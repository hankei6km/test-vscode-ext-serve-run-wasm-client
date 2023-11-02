import * as http from "node:http";

function dataToNumberArray(data) {
  return Array.from(
    (() => {
      if (typeof data === "string") {
        return new TextEncoder().encode(data);
      } else if (Array.isArray(data)) {
        return new Uint8Array(data);
      } else if (data instanceof Uint8Array) {
        return data;
      }
      throw new Error("invalid data type");
    })()
  );
}

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
  if (req.method === "POST") {
    const s1 = JSON.stringify({ id: 0, data: dataToNumberArray(r.route) });
    await new Promise((resolve) => res.write(s1.slice(0, 3), resolve));
    await new Promise((resolve) => res.write(s1.slice(3), resolve));

    const s2 = JSON.stringify({
      id: 1,
      data: dataToNumberArray(JSON.stringify(r.args)),
    });
    await new Promise((resolve) => res.write(s2.slice(0, 3), resolve));
    await new Promise((resolve) => res.write(s2.slice(3), resolve));

    // handle data from request
    for await (const chunk of req) {
      //  chunk to UInt8Array
      const s = JSON.stringify({ id: 2, data: dataToNumberArray(chunk) });
      await new Promise((resolve) => res.write(s, resolve));
    }
    res.end();
  } else {
    res.end("only POST method is allowed");
  }
  server.close();
});

server.listen(ipcHandlePath || 3000);
