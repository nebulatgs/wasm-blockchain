import type Peer from "peerjs";
import React, { useEffect, useRef, useState } from "react";
import Avatar from "react-avatar";
import { uniqueNamesGenerator, adjectives, colors, names } from 'unique-names-generator';
import initWasm, {
	add_block_to_holder,
	add_chain_to_holder,
	Chainholder,
	get_chain_from_holder,
	setup,
	submit_block_to_holder,
	mine_block
} from "wasm";
import { RelativeTime } from '../components/relativeTime'
interface Block {
	previous_hash: string;
	timestamp: number;
	msg: Message;
	nonce: number;
}

interface Message {
	sender: {
		name: string;
		public: unknown;
	}
	data: string;
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
	timestamp: string;
}
export default function Home() {
	const [message, setMessage] = useState("");
	const [blocks, setBlocks] = useState<Block[]>([]);
	// const [holder, setHolder] = useState<Chainholder>();
	let holder = useRef<Chainholder>();
	let conns = useRef<Peer.DataConnection[]>([]);
	// let peer: Peer;
	let peer = useRef<Peer>();
	const AlwaysScrollToBottom = () => {
		const elementRef = useRef<HTMLDivElement>(null);
		useEffect(() => elementRef?.current?.scrollIntoView());
		return <div ref={elementRef} />;
	};
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
			holder.current = setup(uniqueNamesGenerator({
				length: 2,
				separator: '',
				dictionaries: [adjectives, colors, names]
			}));
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
					.map((c) => conns.current.push(c));
				conns.current.forEach((c) => c.on("data", recv));
				console.log(conns);
			});
			peer.current.on("connection", (conn) => {
				conn.on("data", recv);
				conn.on("open", () => send_chain(conn));
				// send_chain(conn);
				conns.current.push(conn);
			});
		})();
	}, []);

	async function submit(data: string) {
		setMessage("");
		const wasm_block = await withHolder(mine_block, data);
		const block = JSON.parse(withHolder(submit_block_to_holder, wasm_block)) as Block;
		console.log(conns);
		conns.current.forEach((conn) => conn.send(packageData("block", block)))
		setBlocks(getBlocks());
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
				setBlocks(getBlocks());
				// displayBlock(data.data as Block);
				break;
			case "chain":
				withHolder(add_chain_to_holder, JSON.stringify(data.data as Chain));
				setBlocks(getBlocks());
				break;
		}
	}
	const getBlocks = () =>
		(JSON.parse(withHolder(get_chain_from_holder)) as Chain).chain;
	function packageData<T extends SendableKey>(
		kind: T,
		data: Sendable[T]
	): Package<Sendable[T]> {
		return {
			kind: kind,
			data: data,
			timestamp: new Date().toISOString(),
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
		<div className="bg-gray-700 h-screen w-full flex flex-col">
			<div className="mt-auto overflow-auto scrollbar scrollbar-thumb-gray-400 scrollbar-track-gray-700 scrollbar-thumb-rounded-full">
				{blocks.map(block => (
					<div key={block.timestamp} className="m">
						<div className="flex w-full p-3">
							<Avatar round={true} size="3em" className="mr-3" name={block.msg.sender.name} />
							<div className="flex flex-col font-sans">
								<div className="flex">
									<span className="mr-3 text-gray-200 font-bold text-lg inline">{block.msg.sender.name}</span>
									<div className="text-gray-400 text-sm">
										<RelativeTime date={new Date(block.timestamp)} />
									</div>
								</div>
								<span className="text-gray-100 sm:max-w-lg md:max-w-2xl lg:max-w-4xl max-w-xs break-words">
									{block.msg.data}
								</span>
							</div>
						</div>
					</div>
				))}
				<AlwaysScrollToBottom />
			</div>
			<div className="flex w-full">
				<input
					type='text'
					className="w-full h-10 p-5 m-3 rounded-full focus:outline-none bg-gray-600 text-gray-100"
					placeholder="Submit a message to the blockchain"
					onChange={(m) => setMessage(m.target.value)}
					value={message}
					onKeyPress={(k) => { k.key === 'Enter' ? submit(message) : null }}
				/>
				{/* <button onClick={() => submit(message)}>Submit Block</button> */}
			</div>
		</div>
	);
}
