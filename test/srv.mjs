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

server.once("request", (req, res) => {
  const r = getRouteAndArgs(req.url);

  res.write(JSON.stringify({ id: 0, data: r.route }));
  res.end(JSON.stringify({ id: 1, data: r.args }));
  server.close();
});

server.listen(ipcHandlePath || 3000);
