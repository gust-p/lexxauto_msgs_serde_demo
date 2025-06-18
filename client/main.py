import time
import zenoh
import zenoh_chatter_demo as lib


def call_service_once(session, data_value):
    """Call the service once with the given boolean value"""
    # Create a SetBool request
    request = lib.SetBoolSrvRequest(data=data_value)
    serialized = request.serialize()

    print(f"Sending SetBool request: data={data_value}")
    print(f"Serialized: {serialized.hex()}")
    print(f"Length: {len(serialized)} bytes")

    # Call the service using Zenoh get (this is the service call pattern)
    service_key = "demo/set_bool"

    try:
        print(f"Calling service at: {service_key}")

        # Use get() to call the service
        replies = session.get(
            service_key, payload=serialized, encoding="application/cdr", timeout=5.0
        )

        print("Waiting for replies...")

        # Process replies
        reply_count = 0
        for reply in replies:
            reply_count += 1
            print(f"� Reply {reply_count} received from: {reply.ok.key_expr}")

            # Get response data
            response_data = bytes(reply.ok.payload.to_bytes())
            print(f"Response data ({len(response_data)} bytes): {response_data.hex()}")

            # Try to deserialize
            try:
                response = lib.SetBoolSrvResponse.deserialize(list(response_data))
                print(
                    f"✅ Parsed response: success={response.success}, message='{response.message}'"
                )
                return True
            except Exception as e:
                print(f"❌ Failed to parse response: {e}")
                print(f"Raw data: {list(response_data)}")
                return False

        if reply_count == 0:
            print("❌ No replies received - service might not be available")
            return False
        else:
            print(f"✅ Received {reply_count} reply(ies)")
            return True

    except Exception as e:
        print(f"❌ Service call failed: {e}")
        return False


def main():
    print("� SetBool Service Test (using Zenoh get)")

    with zenoh.open(zenoh.Config()) as session:
        print("✓ Zenoh session opened")

        # Wait for session to be ready
        time.sleep(1)

        try:
            # Initial test with a few values
            print("\n--- Automated Tests ---")
            test_values = [True, False, True]

            for i, value in enumerate(test_values):
                print(f"\n� Test {i + 1}: Setting bool to {value}")
                success = call_service_once(session, value)
                if success:
                    print(f"✅ Test {i + 1} completed successfully")
                else:
                    print(f"❌ Test {i + 1} failed")
                time.sleep(1)  # Brief pause between tests

            # Interactive mode
            print("\n--- Interactive Mode ---")
            print("Commands:")
            print("  'true' or '1'  - Call service with data=true")
            print("  'false' or '0' - Call service with data=false")
            print("  'quit' or 'q'  - Exit the program")
            print("  'help' or 'h'  - Show this help")

            while True:
                try:
                    user_input = input("\n> ").strip().lower()

                    if user_input in ["quit", "q", "exit"]:
                        print("� Goodbye!")
                        break
                    elif user_input in ["help", "h", "?"]:
                        print(
                            "Commands: 'true'/'1', 'false'/'0', 'quit'/'q', 'help'/'h'"
                        )
                    elif user_input in ["true", "1", "yes", "on"]:
                        print("\n� Calling service with data=TRUE")
                        call_service_once(session, True)
                    elif user_input in ["false", "0", "no", "off"]:
                        print("\n� Calling service with data=FALSE")
                        call_service_once(session, False)
                    elif user_input == "":
                        continue  # Just pressed enter, ignore
                    else:
                        print("❓ Unknown command. Type 'help' for available commands.")

                except EOFError:
                    print("\n� Goodbye!")
                    break
                except KeyboardInterrupt:
                    print("\n� Goodbye!")
                    break

        except KeyboardInterrupt:
            print("\n✓ Shutting down...")


if __name__ == "__main__":
    main()
