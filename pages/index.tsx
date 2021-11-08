import type Peer from "peerjs";
import { useEffect, useRef, useState } from "react";
import initWasm, {
	add_block_to_holder,
	add_chain_to_holder,
	Chainholder,
	get_chain_from_holder,
	setup,
	submit_block_to_holder,
} from "wasm";
interface Block {
	previous_hash: string;
	timestamp: number;
	data: string;
	nonce: number;
}
interface Chain {
	chain: Block[];
}

type Sendable = {
	block: Block;
	chain: Chain;
};
type SendableValue = Sendable[keyof Sendable];
type SendableKey = keyof Sendable;
interface Package<T extends Sendable[keyof Sendable]> {
	kind: keyof Sendable;
	data: T;
	timestamp: number;
}
export default function Home() {
	const [message, setMessage] = useState("");
	const [blocks, setBlocks] = useState<Block[]>([]);
	// const [holder, setHolder] = useState<Chainholder>();
	let holder = useRef<Chainholder>();
	let conns: Peer.DataConnection[] = [];
	// let peer: Peer;
	let peer = useRef<Peer>();

	const withHolder = <T,>(
		f: (holder: Chainholder, ...args: any[]) => T,
		...args: any[]
	): T => {
		if (!holder.current) {
			throw new Error("No holder");
		}
		return f(holder.current, ...args);
	};

	useEffect(() => {
		(async () => {
			const Peer = (await import("peerjs")).default;
			await initWasm();
			holder.current = setup();
			peer.current = new Peer({
				key: "peerjs",
				host: "peerjs-server-production.up.railway.app",
			});
			peer.current.on("open", async (id) => {
				// alert('My peer ID is: ' + id);
				const peers = await getAllPeers();
				peers
					.filter((p) => p != id)
					.map((p) => peer.current!.connect(p, { reliable: true }))
					.map((c) => conns.push(c));
				conns.forEach((c) => c.on("data", recv));
				console.log(conns);
			});
			peer.current.on("connection", (conn) => {
				conn.on("data", recv);
				conn.on("open", () => send_chain(conn));
				// send_chain(conn);
				conns.push(conn);
			});
		})();
	}, []);

	function submit(data: string) {
		const block = JSON.parse(withHolder(submit_block_to_holder, data)) as Block;
		// displayBlock(block);
		console.log(
			conns.forEach((conn) => conn.send(packageData("block", block)))
		);
	}
	// function verify() {
	// 	console.log(withHolder(verify_chain_in_holder));
	// }
	// function displayBlock(block: Block) {
	// 	if (block) {
	// 		document.querySelector("body").append(document.createElement("br"));
	// 		document
	// 			.querySelector("body")
	// 			.append(`${block.previous_hash} -> ${block.data}`);
	// 	}
	// }
	// function displayChain(chain: Chain) {
	// 	chain.chain.forEach(displayBlock);
	// }
	function recv<T extends SendableValue>(data: Package<T>) {
		console.log(data);
		switch (data.kind) {
			case "block":
				withHolder(add_block_to_holder, JSON.stringify(data.data));
				// displayBlock(data.data as Block);
				break;
			case "chain":
				withHolder(add_chain_to_holder, JSON.stringify(data.data as Chain));
				break;
		}
	}
	function packageData<T extends SendableKey>(
		kind: T,
		data: Sendable[T]
	): Package<Sendable[T]> {
		return {
			kind: kind,
			data: data,
			timestamp: new Date().getUTCMilliseconds(),
		};
	}
	function send_chain(conn: Peer.DataConnection) {
		console.log(conn);
		console.log(withHolder(get_chain_from_holder));
		conn.send(
			packageData(
				"chain",
				JSON.parse(withHolder(get_chain_from_holder)) as Chain
			)
		);
	}
	async function getAllPeers() {
		return (await fetch(
			"https://peerjs-server-production.up.railway.app/peerjs/peers"
		).then((res) => res.json())) as string[];
	}

	return (
		<div>
			<input type='text' onChange={(m) => setMessage(m.target.value)} />
			<button onClick={() => submit(message)}>Submit Block</button>
		</div>
	);
}
