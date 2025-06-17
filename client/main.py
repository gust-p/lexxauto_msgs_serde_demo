import time

import zenoh
import zenoh_chatter_demo as lib
import numpy as np


def main():
    with zenoh.open(zenoh.Config()) as session:
        x = np.float64(1.0)
        y = np.float64(1.0)
        z = np.float64(1.0)
        message = lib.Vector3(x=x, y=y, z=z)
        serialized = message.serialize()
        while True:
            print(f"Sending message: {serialized.hex()}")
            print(f"Message length: {len(serialized)}")
            session.put("demo/hello", serialized, encoding="application/cdr")
            time.sleep(1)


if __name__ == "__main__":
    main()
