
import ws from 'k6/ws';
import { check } from 'k6';

let rooms = 10;

let messages = 10;
let target_vus = 60;

export const options = {
    stages: [
        { duration: '30s', target: target_vus },
        { duration: '60s', target: target_vus },
        { duration: '5s', target: target_vus },
    ],
};


let run = 0;

export default function () {

    let room = "room-" + Math.floor(rooms % (run++));

    const url = 'ws://localhost:6789';
    let user = "loadtest-" + Math.random();
    const res = ws.connect(url, undefined, function (socket) {
        socket.send(JSON.stringify({
            room,
            user,
        }))

        socket.setTimeout(() => {
            for (let i = 0; i < messages; i++) {
                socket.send(JSON.stringify({
                    text: "test: " + i,
                }))
            }
        }, 1000);

        let received = 0;

        socket.on('message', (data) => {
            data = JSON.parse(data);
            if (data.user === user && data.text.startsWith("test")) {
                received++;
                if (received === messages) {
                    socket.close();
                }
            }
        });
    });

    check(res, { 'status is 101': (r) => r && r.status === 101 });
}