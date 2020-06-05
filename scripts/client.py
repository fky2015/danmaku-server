import websocket
import json
try:
    import thread
except ImportError:
    import _thread as thread
import time


def on_message(ws, message):
    print(message)


def on_error(ws, error):
    print(error)


def on_close(ws):
    print("### closed ###")


def on_open(ws):
    def run(*args):
        while True:
            text = input("input your message\n")
            d = json.dumps(
                     {
                    "type": 0,
                    "text": text,
                    "color": 0x555555,
                    }
                    )
            data = json.dumps({
                    "type": "Danmaku",
                    "data": d
                })
            print(data)
            ws.send(data)
        time.sleep(1)
        ws.close()
        print("thread terminating...")
    thread.start_new_thread(run, ())


if __name__ == "__main__":
    websocket.enableTrace(False)
    ws = websocket.WebSocketApp("ws://localhost:8080/ws/",
                              on_message=on_message,
                              on_error=on_error,
                              on_close=on_close)
    ws.on_open = on_open
    ws.run_forever()
