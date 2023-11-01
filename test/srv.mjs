import * as http from "node:http";

function getRouteAndArgs(url) {
  const p = (() => {
    const p = url.split("?", 2);
    return p.length > 1 ? p : [p[0], ""];
  })();
  const parsed = new URLSearchParams(p[1]);
  const args = (() => {
    const argsStr = parsed.get("args");
    if (argsStr) {
      try {
        const a = JSON.parse(argsStr);
        if (Array.isArray(a)) {
          return a.map((v) => `${v}`);
        }
      } catch (e) {}
    }
    return [];
  })();
  return {
    route: p[0],
    args,
  };
}

const ipcHandlePath = process.env["IPC_HANDLE_PATH"];

const server = http.createServer();

server.once("request", async (req, res) => {
  const r = getRouteAndArgs(req.url);

  const s1 = JSON.stringify({ id: 0, data: r.route });
  await new Promise((resolve) => res.write(s1.slice(0, 3), resolve));
  await new Promise((resolve) => res.write(s1.slice(3), resolve));

  const s2 = JSON.stringify({ id: 1, data: r.args });
  await new Promise((resolve) => res.write(s2.slice(0, 3), resolve));
  await new Promise((resolve) => res.write(s2.slice(3), resolve));

  // handle data from request
  for await (const chunk of req) {
    //  chunk to UInt8Array
    const data = new Uint8Array(chunk);
    const s = JSON.stringify({ id: 2, data: Array.from(data) });
    await new Promise((resolve) => res.write(s, resolve));
  }
  res.end();
  server.close();
});

server.listen(ipcHandlePath || 3000);
