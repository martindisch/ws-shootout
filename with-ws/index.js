import { WebSocketServer } from "ws";

const connections = new Set();

const wss = new WebSocketServer({ port: 8080 });

wss.on("connection", (ws) => {
  connections.add(ws);

  ws.on("close", () => connections.delete(ws));
});

const periodicUpdate = () => {
  const date = new Date().toString();
  connections.forEach((ws) => ws.send(date));

  setTimeout(periodicUpdate, 1000);
};

periodicUpdate();
