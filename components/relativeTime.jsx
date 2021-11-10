import { DateTime } from "luxon";

let i = {};
i.pad = function(e, t, n, r) {
    e += "";
    n ? n.length > 1 && (n = n.charAt(0)) : n = " ";
    if ("right" === (r = void 0 === r ? "left" : "right"))
        for (; e.length < t;) e += n;
    else
        for (; e.length < t;) e = n + e;
    return e
};
i.time = function() {
    return (new Date).getTime() / 1e3
};
i.date = function(e, t) {
    var a = [0, 0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
        o = [0, 0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];
    var n = void 0 === t ? new Date : t instanceof Date ? new Date(t) : new Date(1e3 * t),
        r = /\\?([a-z])/gi,
        s = function(e, t) {
            return c[e] ? c[e]() : t
        },
        u = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"],
        l = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
        c = {
            d: function() {
                return i.pad(c.j(), 2, "0")
            },
            D: function() {
                return c.l().slice(0, 3)
            },
            j: function() {
                return n.getDate()
            },
            l: function() {
                return u[c.w()]
            },
            N: function() {
                return c.w() || 7
            },
            S: function() {
                var e = c.j();
                return e > 4 && e < 21 ? "th" : {
                    1: "st",
                    2: "nd",
                    3: "rd"
                } [e % 10] || "th"
            },
            w: function() {
                return n.getDay()
            },
            z: function() {
                return (c.L() ? o[c.n()] : a[c.n()]) + c.j() - 1
            },
            W: function() {
                var e = c.z() - c.N() + 1.5;
                return i.pad(1 + Math.floor(Math.abs(e) / 7) + (e % 7 > 3.5 ? 1 : 0), 2, "0")
            },
            F: function() {
                return l[n.getMonth()]
            },
            m: function() {
                return i.pad(c.n(), 2, "0")
            },
            M: function() {
                return c.F().slice(0, 3)
            },
            n: function() {
                return n.getMonth() + 1
            },
            t: function() {
                return new Date(c.Y(), c.n(), 0).getDate()
            },
            L: function() {
                return 1 === new Date(c.Y(), 1, 29).getMonth() ? 1 : 0
            },
            o: function() {
                var e = c.n(),
                    t = c.W();
                return c.Y() + (12 === e && t < 9 ? -1 : 1 === e && t > 9)
            },
            Y: function() {
                return n.getFullYear()
            },
            y: function() {
                return String(c.Y()).slice(-2)
            },
            a: function() {
                return n.getHours() > 11 ? "pm" : "am"
            },
            A: function() {
                return c.a().toUpperCase()
            },
            B: function() {
                var e = n.getTime() / 1e3,
                    t = e % 86400 + 3600;
                t < 0 && (t += 86400);
                var r = t / 86.4 % 1e3;
                return e < 0 ? Math.ceil(r) : Math.floor(r)
            },
            g: function() {
                return c.G() % 12 || 12
            },
            G: function() {
                return n.getHours()
            },
            h: function() {
                return i.pad(c.g(), 2, "0")
            },
            H: function() {
                return i.pad(c.G(), 2, "0")
            },
            i: function() {
                return i.pad(n.getMinutes(), 2, "0")
            },
            s: function() {
                return i.pad(n.getSeconds(), 2, "0")
            },
            u: function() {
                return i.pad(1e3 * n.getMilliseconds(), 6, "0")
            },
            O: function() {
                var e = n.getTimezoneOffset(),
                    t = Math.abs(e);
                return (e > 0 ? "-" : "+") + i.pad(100 * Math.floor(t / 60) + t % 60, 4, "0")
            },
            P: function() {
                var e = c.O();
                return e.substr(0, 3) + ":" + e.substr(3, 2)
            },
            Z: function() {
                return 60 * -n.getTimezoneOffset()
            },
            c: function() {
                return "Y-m-d\\TH:i:sP".replace(r, s)
            },
            r: function() {
                return "D, d M Y H:i:s O".replace(r, s)
            },
            U: function() {
                return n.getTime() / 1e3 || 0
            }
        };
    return e.replace(r, s)
};
i.relativeTime = function(e) {
    e = void 0 === e ? i.time() : e;
    var t = i.time(),
        n = t - e;
    if (n < 2 && n > -2) return (n >= 0 ? "just " : "") + "now";
    if (n < 60 && n > -60) return n >= 0 ? Math.floor(n) + " seconds ago" : "in " + Math.floor(-n) + " seconds";
    if (n < 120 && n > -120) return n >= 0 ? "about a minute ago" : "in about a minute";
    if (n < 3600 && n > -3600) return n >= 0 ? Math.floor(n / 60) + " minutes ago" : "in " + Math.floor(-n / 60) + " minutes";
    if (n < 7200 && n > -7200) return n >= 0 ? "about an hour ago" : "in about an hour";
    if (n < 86400 && n > -86400) return n >= 0 ? Math.floor(n / 3600) + " hours ago" : "in " + Math.floor(-n / 3600) + " hours";
    var r = 172800;
    if (n < r && n > -r) return n >= 0 ? "1 day ago" : "in 1 day";
    var a = 2505600;
    if (n < a && n > -a) return n >= 0 ? Math.floor(n / 86400) + " days ago" : "in " + Math.floor(-n / 86400) + " days";
    var o = 5184e3;
    if (n < o && n > -o) return n >= 0 ? "about a month ago" : "in about a month";
    var s = parseInt(i.date("Y", t), 10),
        u = parseInt(i.date("Y", e), 10),
        l = 12 * s + parseInt(i.date("n", t), 10) - (12 * u + parseInt(i.date("n", e), 10));
    if (l < 12 && l > -12) return l >= 0 ? l + " months ago" : "in " + -l + " months";
    var c = s - u;
    return c < 2 && c > -2 ? c >= 0 ? "a year ago" : "in a year" : c >= 0 ? c + " years ago" : "in " + -c + " years"
};

export function RelativeTime({ date }) {
    const dateTime = DateTime.fromJSDate(date);
    return (
        <>
            {i.relativeTime(date.getTime() / 1e3)}
        </>
    );
}