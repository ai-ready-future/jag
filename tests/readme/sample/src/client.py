import time

max_retry = 3
retry_wait = 2


def send(request):
    for attempt in range(max_retry):
        reply = call_service(request)
        if reply.ok:
            return reply
        # Wait before the next try.
        time.sleep(retry_wait)
    raise RuntimeError("the service did not answer")
