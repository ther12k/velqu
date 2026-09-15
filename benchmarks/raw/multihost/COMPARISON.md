# Multi-host benchmark comparison (descriptive)

Matched protocol per host: fixed seed, 10s cells, 5 repetitions, c=1/10/50, all candidates interleaved in one harness process inside a CPU-limited container. Hosts differ in CPU/kernel/virtualization; numbers are comparable ACROSS candidates within a host, and across hosts only as separate environments. Generated from raw JSONL in benchmarks/raw/multihost.

## multihost-halotec-20260915T181508Z

| candidate | route | c | rps (med) | p95 (med) |
|---|---|---:|---:|---:|
| velqu | C0 | 1 | 3891 | 470us |
| raw-rust | C0 | 1 | 3868 | 481us |
| raw-bun | C0 | 1 | 3697 | 509us |
| elysia2 | C0 | 1 | 3698 | 495us |
| lugas | C0 | 1 | 3821 | 480us |
| velqu | C0 | 10 | 16745 | 1196us |
| raw-rust | C0 | 10 | 18205 | 1062us |
| raw-bun | C0 | 10 | 14870 | 1543us |
| elysia2 | C0 | 10 | 16261 | 1323us |
| lugas | C0 | 10 | 15219 | 1503us |
| velqu | C0 | 50 | 21496 | 5037us |
| raw-rust | C0 | 50 | 23022 | 4599us |
| raw-bun | C0 | 50 | 20872 | 5631us |
| elysia2 | C0 | 50 | 21398 | 5083us |
| lugas | C0 | 50 | 22522 | 4865us |
| velqu | C1 | 1 | 2262 | 757us |
| raw-rust | C1 | 1 | 4195 | 427us |
| raw-bun | C1 | 1 | 3707 | 491us |
| elysia2 | C1 | 1 | 3573 | 515us |
| lugas | C1 | 1 | 3685 | 509us |
| velqu | C1 | 10 | 10280 | 1829us |
| raw-rust | C1 | 10 | 18012 | 1057us |
| raw-bun | C1 | 10 | 16191 | 1286us |
| elysia2 | C1 | 10 | 16113 | 1376us |
| lugas | C1 | 10 | 17434 | 1149us |
| velqu | C1 | 50 | 13723 | 8906us |
| raw-rust | C1 | 50 | 22341 | 4801us |
| raw-bun | C1 | 50 | 20256 | 5671us |
| elysia2 | C1 | 50 | 21426 | 5386us |
| lugas | C1 | 50 | 22338 | 4786us |
| velqu | C2 | 1 | 3845 | 463us |
| raw-rust | C2 | 1 | 4145 | 428us |
| raw-bun | C2 | 1 | 3652 | 501us |
| elysia2 | C2 | 1 | 3722 | 482us |
| lugas | C2 | 1 | 3859 | 467us |
| velqu | C2 | 10 | 16502 | 1211us |
| raw-rust | C2 | 10 | 17608 | 1121us |
| raw-bun | C2 | 10 | 16342 | 1229us |
| elysia2 | C2 | 10 | 15719 | 1374us |
| lugas | C2 | 10 | 15448 | 1463us |
| velqu | C2 | 50 | 21097 | 5014us |
| raw-rust | C2 | 50 | 22444 | 4802us |
| raw-bun | C2 | 50 | 21018 | 5113us |
| elysia2 | C2 | 50 | 21783 | 4840us |
| lugas | C2 | 50 | 22110 | 4699us |
| velqu | C3 | 1 | 2203 | 760us |
| raw-rust | C3 | 1 | 3973 | 461us |
| raw-bun | C3 | 1 | 3667 | 490us |
| elysia2 | C3 | 1 | 3607 | 497us |
| lugas | C3 | 1 | 3811 | 474us |
| velqu | C3 | 10 | 8710 | 2599us |
| raw-rust | C3 | 10 | 17313 | 1143us |
| raw-bun | C3 | 10 | 15952 | 1257us |
| elysia2 | C3 | 10 | 16173 | 1212us |
| lugas | C3 | 10 | 16467 | 1236us |
| velqu | C3 | 50 | 12534 | 10013us |
| raw-rust | C3 | 50 | 21290 | 5718us |
| raw-bun | C3 | 50 | 20640 | 5614us |
| elysia2 | C3 | 50 | 20750 | 5412us |
| lugas | C3 | 50 | 19780 | 6007us |

### velqu as % of baseline (halotec-20260915T181508Z, median rps)

| route/c | raw-rust | raw-bun | elysia2 | lugas |
|---|---:|---:|---:|---:|
| C0 c=1 | 101% | 105% | 105% | 102% |
| C0 c=10 | 92% | 113% | 103% | 110% |
| C0 c=50 | 93% | 103% | 100% | 95% |
| C1 c=1 | 54% | 61% | 63% | 61% |
| C1 c=10 | 57% | 63% | 64% | 59% |
| C1 c=50 | 61% | 68% | 64% | 61% |
| C2 c=1 | 93% | 105% | 103% | 100% |
| C2 c=10 | 94% | 101% | 105% | 107% |
| C2 c=50 | 94% | 100% | 97% | 95% |
| C3 c=1 | 55% | 60% | 61% | 58% |
| C3 c=10 | 50% | 55% | 54% | 53% |
| C3 c=50 | 59% | 61% | 60% | 63% |

## multihost-oracle-20260915T183804Z

| candidate | route | c | rps (med) | p95 (med) |
|---|---|---:|---:|---:|
| velqu | C0 | 1 | 12179 | 103us |
| raw-rust | C0 | 1 | 13298 | 93us |
| raw-bun | C0 | 1 | 10381 | 121us |
| elysia2 | C0 | 1 | 10283 | 121us |
| lugas | C0 | 1 | 9849 | 128us |
| velqu | C0 | 10 | 26097 | 556us |
| raw-rust | C0 | 10 | 33710 | 388us |
| raw-bun | C0 | 10 | 20806 | 723us |
| elysia2 | C0 | 10 | 21060 | 728us |
| lugas | C0 | 10 | 21710 | 691us |
| velqu | C0 | 50 | 33547 | 2061us |
| raw-rust | C0 | 50 | 40770 | 1869us |
| raw-bun | C0 | 50 | 22454 | 3346us |
| elysia2 | C0 | 50 | 22946 | 3296us |
| lugas | C0 | 50 | 23415 | 3192us |
| velqu | C1 | 1 | 6402 | 189us |
| raw-rust | C1 | 1 | 10369 | 102us |
| raw-bun | C1 | 1 | 8826 | 128us |
| elysia2 | C1 | 1 | 10148 | 124us |
| lugas | C1 | 1 | 11054 | 114us |
| velqu | C1 | 10 | 15054 | 996us |
| raw-rust | C1 | 10 | 34723 | 388us |
| raw-bun | C1 | 10 | 21203 | 710us |
| elysia2 | C1 | 10 | 20263 | 754us |
| lugas | C1 | 10 | 21981 | 685us |
| velqu | C1 | 50 | 20033 | 3624us |
| raw-rust | C1 | 50 | 42137 | 1626us |
| raw-bun | C1 | 50 | 23781 | 3105us |
| elysia2 | C1 | 50 | 22601 | 3343us |
| lugas | C1 | 50 | 23685 | 3133us |
| velqu | C2 | 1 | 10981 | 107us |
| raw-rust | C2 | 1 | 13738 | 90us |
| raw-bun | C2 | 1 | 9943 | 125us |
| elysia2 | C2 | 1 | 10382 | 119us |
| lugas | C2 | 1 | 10780 | 118us |
| velqu | C2 | 10 | 27368 | 516us |
| raw-rust | C2 | 10 | 34329 | 388us |
| raw-bun | C2 | 10 | 20302 | 746us |
| elysia2 | C2 | 10 | 20428 | 758us |
| lugas | C2 | 10 | 21773 | 689us |
| velqu | C2 | 50 | 33808 | 2026us |
| raw-rust | C2 | 50 | 40565 | 1781us |
| raw-bun | C2 | 50 | 23053 | 3277us |
| elysia2 | C2 | 50 | 21065 | 3616us |
| lugas | C2 | 50 | 22658 | 3258us |
| velqu | C3 | 1 | 5935 | 200us |
| raw-rust | C3 | 1 | 13212 | 95us |
| raw-bun | C3 | 1 | 9981 | 127us |
| elysia2 | C3 | 1 | 9798 | 126us |
| lugas | C3 | 1 | 9683 | 129us |
| velqu | C3 | 10 | 12200 | 1440us |
| raw-rust | C3 | 10 | 32724 | 405us |
| raw-bun | C3 | 10 | 19350 | 782us |
| elysia2 | C3 | 10 | 19523 | 802us |
| lugas | C3 | 10 | 19182 | 804us |
| velqu | C3 | 50 | 16308 | 4614us |
| raw-rust | C3 | 50 | 40451 | 1752us |
| raw-bun | C3 | 50 | 20494 | 3994us |
| elysia2 | C3 | 50 | 20397 | 4658us |
| lugas | C3 | 50 | 21492 | 3468us |

### velqu as % of baseline (oracle-20260915T183804Z, median rps)

| route/c | raw-rust | raw-bun | elysia2 | lugas |
|---|---:|---:|---:|---:|
| C0 c=1 | 92% | 117% | 118% | 124% |
| C0 c=10 | 77% | 125% | 124% | 120% |
| C0 c=50 | 82% | 149% | 146% | 143% |
| C1 c=1 | 62% | 73% | 63% | 58% |
| C1 c=10 | 43% | 71% | 74% | 68% |
| C1 c=50 | 48% | 84% | 89% | 85% |
| C2 c=1 | 80% | 110% | 106% | 102% |
| C2 c=10 | 80% | 135% | 134% | 126% |
| C2 c=50 | 83% | 147% | 160% | 149% |
| C3 c=1 | 45% | 59% | 61% | 61% |
| C3 c=10 | 37% | 63% | 62% | 64% |
| C3 c=50 | 40% | 80% | 80% | 76% |

