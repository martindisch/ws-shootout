import { WebSocketServer } from "ws";

const clients = new Set();

const wss = new WebSocketServer({ port: 8080 });

wss.on("connection", (ws) => {
  clients.add(ws);

  ws.on("close", () => clients.delete(ws));
});

const periodicUpdate = () => {
  const date = new Date().toString();
  console.log(`Sending date to ${clients.size} clients`);
  clients.forEach((ws) => ws.send(date));

  setTimeout(periodicUpdate, 1000);
};

periodicUpdate();
