def run_jobs(jobs):
    for job in jobs:
        try:
            job.run()
        except TimeoutError:
            job.retry_count += 1
            jobs.append(job)
