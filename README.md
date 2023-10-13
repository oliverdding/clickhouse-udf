# clickhouse udf - neoTopK

## Develop

```bash
cargo build
docker compose up -d
```

## Broken pipe

```
clickhouse :) select neoTopK(2)(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc']));

SELECT neoTopK(2)(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc']))

Query id: 30c97994-41a3-47d7-9481-63b0190e728c

[clickhouse] 2023.10.13 17:20:01.720414 [ 50 ] {30c97994-41a3-47d7-9481-63b0190e728c} <Debug> executeQuery: (from 127.0.0.1:49416) select neoTopK(2)(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc'])); (stage: Complete)
[clickhouse] 2023.10.13 17:20:01.720931 [ 50 ] {30c97994-41a3-47d7-9481-63b0190e728c} <Trace> ContextAccess (default): Access granted: SELECT(dummy) ON system.one
[clickhouse] 2023.10.13 17:20:01.720992 [ 50 ] {30c97994-41a3-47d7-9481-63b0190e728c} <Trace> InterpreterSelectQuery: FetchColumns -> Complete
[clickhouse] 2023.10.13 17:20:01.721771 [ 410 ] {30c97994-41a3-47d7-9481-63b0190e728c} <Trace> ShellCommand: Will start shell command '/var/lib/clickhouse/user_scripts//clickhouse-udf-neo-top-k' with arguments '/var/lib/clickhouse/user_scripts//clickhouse-udf-neo-top-k', '2'
[clickhouse] 2023.10.13 17:20:01.726134 [ 410 ] {30c97994-41a3-47d7-9481-63b0190e728c} <Trace> ShellCommand: Started shell command '/var/lib/clickhouse/user_scripts//clickhouse-udf-neo-top-k' with pid 907
[clickhouse] 2023.10.13 17:20:01.728571 [ 410 ] {30c97994-41a3-47d7-9481-63b0190e728c} <Trace> ShellCommand: Try wait for shell command pid 907 with timeout 10
[clickhouse] 2023.10.13 17:20:01.729056 [ 50 ] {30c97994-41a3-47d7-9481-63b0190e728c} <Error> executeQuery: Code: 75. DB::ErrnoException: Cannot write into pipe, errno: 32, strerror: Broken pipe: While executing BinaryRowOutputFormat: while executing 'FUNCTION neoTopK(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc']) :: 1) -> neoTopK(2)(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc'])) Array(Tuple(String, UInt64)) : 0'. (CANNOT_WRITE_TO_FILE_DESCRIPTOR) (version 23.4.6.25 (official build)) (from 127.0.0.1:49416) (in query: select neoTopK(2)(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc']));), Stack trace (when copying this message, always include the lines below):

0. DB::Exception::Exception(DB::Exception::MessageMasked&&, int, bool) @ 0xbcf0be4 in /usr/bin/clickhouse
1. ? @ 0xbcf1720 in /usr/bin/clickhouse
2. DB::throwFromErrno(String const&, int, int) @ 0xbcf15c4 in /usr/bin/clickhouse
3. ? @ 0x118fe9c4 in /usr/bin/clickhouse
4. DB::IOutputFormat::flush() @ 0x1167e09c in /usr/bin/clickhouse
5. DB::IOutputFormat::work() @ 0x1167df2c in /usr/bin/clickhouse
6. DB::ExecutionThreadContext::executeTask() @ 0x11670f0c in /usr/bin/clickhouse
7. DB::PipelineExecutor::executeStepImpl(unsigned long, std::atomic<bool>*) @ 0x11668258 in /usr/bin/clickhouse
8. DB::PipelineExecutor::executeImpl(unsigned long) @ 0x116676fc in /usr/bin/clickhouse
9. DB::PipelineExecutor::execute(unsigned long) @ 0x11667354 in /usr/bin/clickhouse
10. DB::CompletedPipelineExecutor::execute() @ 0x11665ab8 in /usr/bin/clickhouse
11. ? @ 0x118ff168 in /usr/bin/clickhouse
12. ? @ 0x11900710 in /usr/bin/clickhouse
13. ThreadPoolImpl<std::thread>::worker(std::__list_iterator<std::thread, void*>) @ 0xbda8354 in /usr/bin/clickhouse
14. ? @ 0xbdad508 in /usr/bin/clickhouse
15. start_thread @ 0x7624 in /lib/libpthread.so.0
16. ? @ 0xd149c in /lib/libc.so.6


0 rows in set. Elapsed: 0.021 sec. 

Received exception from server (version 23.4.6):
Code: 75. DB::Exception: Received from localhost:9000. DB::ErrnoException. DB::ErrnoException: Cannot write into pipe, errno: 32, strerror: Broken pipe: While executing BinaryRowOutputFormat: while executing 'FUNCTION neoTopK(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc']) :: 1) -> neoTopK(2)(arrayJoin(['abc', 'abc', 'ab', 'a', 'b', 'abc'])) Array(Tuple(String, UInt64)) : 0'. (CANNOT_WRITE_TO_FILE_DESCRIPTOR)
```
