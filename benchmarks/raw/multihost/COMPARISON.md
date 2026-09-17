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

## multihost-halotec-20260917T092457Z

| candidate | route | c | rps (med) | p95 (med) |
|---|---|---:|---:|---:|
| velqu | C0 | 1 | 4307 | 406us |
| raw-rust | C0 | 1 | 4608 | 391us |
| raw-bun | C0 | 1 | 4312 | 414us |
| elysia2 | C0 | 1 | 3977 | 445us |
| lugas | C0 | 1 | 4273 | 419us |
| velqu | C0 | 10 | 18153 | 968us |
| raw-rust | C0 | 10 | 19654 | 947us |
| raw-bun | C0 | 10 | 17764 | 1078us |
| elysia2 | C0 | 10 | 17238 | 1250us |
| lugas | C0 | 10 | 17827 | 1078us |
| velqu | C0 | 50 | 22844 | 4819us |
| raw-rust | C0 | 50 | 24709 | 4181us |
| raw-bun | C0 | 50 | 22811 | 4608us |
| elysia2 | C0 | 50 | 22537 | 4910us |
| lugas | C0 | 50 | 23620 | 4058us |
| velqu | C1 | 1 | 4203 | 432us |
| raw-rust | C1 | 1 | 4675 | 373us |
| raw-bun | C1 | 1 | 4186 | 416us |
| elysia2 | C1 | 1 | 4209 | 424us |
| lugas | C1 | 1 | 4262 | 424us |
| velqu | C1 | 10 | 17732 | 1097us |
| raw-rust | C1 | 10 | 20382 | 908us |
| raw-bun | C1 | 10 | 18123 | 974us |
| elysia2 | C1 | 10 | 17131 | 1201us |
| lugas | C1 | 10 | 17679 | 1139us |
| velqu | C1 | 50 | 22499 | 4980us |
| raw-rust | C1 | 50 | 23166 | 4846us |
| raw-bun | C1 | 50 | 23423 | 4175us |
| elysia2 | C1 | 50 | 22805 | 4489us |
| lugas | C1 | 50 | 23076 | 4979us |
| velqu | C2 | 1 | 4451 | 402us |
| raw-rust | C2 | 1 | 4551 | 386us |
| raw-bun | C2 | 1 | 4214 | 420us |
| elysia2 | C2 | 1 | 4244 | 409us |
| lugas | C2 | 1 | 4336 | 413us |
| velqu | C2 | 10 | 18564 | 976us |
| raw-rust | C2 | 10 | 19562 | 1010us |
| raw-bun | C2 | 10 | 17505 | 1144us |
| elysia2 | C2 | 10 | 17202 | 1221us |
| lugas | C2 | 10 | 18316 | 1022us |
| velqu | C2 | 50 | 23774 | 3972us |
| raw-rust | C2 | 50 | 22851 | 4974us |
| raw-bun | C2 | 50 | 21999 | 5150us |
| elysia2 | C2 | 50 | 20853 | 5978us |
| lugas | C2 | 50 | 22526 | 4781us |
| velqu | C3 | 1 | 2698 | 615us |
| raw-rust | C3 | 1 | 4805 | 365us |
| raw-bun | C3 | 1 | 4085 | 425us |
| elysia2 | C3 | 1 | 3991 | 436us |
| lugas | C3 | 1 | 4033 | 442us |
| velqu | C3 | 10 | 11211 | 1753us |
| raw-rust | C3 | 10 | 19478 | 973us |
| raw-bun | C3 | 10 | 17518 | 1119us |
| elysia2 | C3 | 10 | 16776 | 1273us |
| lugas | C3 | 10 | 17188 | 1129us |
| velqu | C3 | 50 | 16093 | 6832us |
| raw-rust | C3 | 50 | 24528 | 3882us |
| raw-bun | C3 | 50 | 22010 | 5032us |
| elysia2 | C3 | 50 | 21523 | 5408us |
| lugas | C3 | 50 | 22629 | 4643us |

### velqu as % of baseline (halotec-20260917T092457Z, median rps)

| route/c | raw-rust | raw-bun | elysia2 | lugas |
|---|---:|---:|---:|---:|
| C0 c=1 | 93% | 100% | 108% | 101% |
| C0 c=10 | 92% | 102% | 105% | 102% |
| C0 c=50 | 92% | 100% | 101% | 97% |
| C1 c=1 | 90% | 100% | 100% | 99% |
| C1 c=10 | 87% | 98% | 104% | 100% |
| C1 c=50 | 97% | 96% | 99% | 97% |
| C2 c=1 | 98% | 106% | 105% | 103% |
| C2 c=10 | 95% | 106% | 108% | 101% |
| C2 c=50 | 104% | 108% | 114% | 106% |
| C3 c=1 | 56% | 66% | 68% | 67% |
| C3 c=10 | 58% | 64% | 67% | 65% |
| C3 c=50 | 66% | 73% | 75% | 71% |

## multihost-local-20260917T092118Z

| candidate | route | c | rps (med) | p95 (med) |
|---|---|---:|---:|---:|
| velqu | C0 | 1 | 15187 | 104us |
| raw-rust | C0 | 1 | 25025 | 62us |
| raw-bun | C0 | 1 | 16541 | 98us |
| elysia2 | C0 | 1 | 17629 | 89us |
| lugas | C0 | 1 | 18707 | 83us |
| velqu | C0 | 10 | 50768 | 244us |
| raw-rust | C0 | 10 | 65284 | 226us |
| raw-bun | C0 | 10 | 44144 | 328us |
| elysia2 | C0 | 10 | 43198 | 316us |
| lugas | C0 | 10 | 58872 | 226us |
| velqu | C0 | 50 | 51545 | 1454us |
| raw-rust | C0 | 50 | 68860 | 898us |
| raw-bun | C0 | 50 | 56103 | 1368us |
| elysia2 | C0 | 50 | 53120 | 1477us |
| lugas | C0 | 50 | 67967 | 1010us |
| velqu | C1 | 1 | 19326 | 80us |
| raw-rust | C1 | 1 | 20320 | 78us |
| raw-bun | C1 | 1 | 18316 | 90us |
| elysia2 | C1 | 1 | 22319 | 72us |
| lugas | C1 | 1 | 16964 | 93us |
| velqu | C1 | 10 | 58838 | 226us |
| raw-rust | C1 | 10 | 64721 | 228us |
| raw-bun | C1 | 10 | 49129 | 282us |
| elysia2 | C1 | 10 | 52768 | 262us |
| lugas | C1 | 10 | 51721 | 259us |
| velqu | C1 | 50 | 56423 | 1217us |
| raw-rust | C1 | 50 | 62214 | 906us |
| raw-bun | C1 | 50 | 51186 | 1641us |
| elysia2 | C1 | 50 | 52474 | 1556us |
| lugas | C1 | 50 | 55317 | 1435us |
| velqu | C2 | 1 | 22035 | 70us |
| raw-rust | C2 | 1 | 21518 | 74us |
| raw-bun | C2 | 1 | 16573 | 102us |
| elysia2 | C2 | 1 | 16997 | 101us |
| lugas | C2 | 1 | 16071 | 113us |
| velqu | C2 | 10 | 48127 | 270us |
| raw-rust | C2 | 10 | 62690 | 227us |
| raw-bun | C2 | 10 | 45316 | 338us |
| elysia2 | C2 | 10 | 44645 | 374us |
| lugas | C2 | 10 | 50634 | 298us |
| velqu | C2 | 50 | 58322 | 1201us |
| raw-rust | C2 | 50 | 68511 | 900us |
| raw-bun | C2 | 50 | 45243 | 1627us |
| elysia2 | C2 | 50 | 49902 | 1536us |
| lugas | C2 | 50 | 67518 | 1148us |
| velqu | C3 | 1 | 11743 | 127us |
| raw-rust | C3 | 1 | 20426 | 70us |
| raw-bun | C3 | 1 | 17942 | 102us |
| elysia2 | C3 | 1 | 18670 | 92us |
| lugas | C3 | 1 | 19764 | 83us |
| velqu | C3 | 10 | 30174 | 273us |
| raw-rust | C3 | 10 | 47856 | 279us |
| raw-bun | C3 | 10 | 53846 | 290us |
| elysia2 | C3 | 10 | 49053 | 298us |
| lugas | C3 | 10 | 55873 | 252us |
| velqu | C3 | 50 | 32869 | 1240us |
| raw-rust | C3 | 50 | 59021 | 1213us |
| raw-bun | C3 | 50 | 65071 | 1118us |
| elysia2 | C3 | 50 | 49883 | 1596us |
| lugas | C3 | 50 | 69008 | 1108us |

### velqu as % of baseline (local-20260917T092118Z, median rps)

| route/c | raw-rust | raw-bun | elysia2 | lugas |
|---|---:|---:|---:|---:|
| C0 c=1 | 61% | 92% | 86% | 81% |
| C0 c=10 | 78% | 115% | 118% | 86% |
| C0 c=50 | 75% | 92% | 97% | 76% |
| C1 c=1 | 95% | 106% | 87% | 114% |
| C1 c=10 | 91% | 120% | 112% | 114% |
| C1 c=50 | 91% | 110% | 108% | 102% |
| C2 c=1 | 102% | 133% | 130% | 137% |
| C2 c=10 | 77% | 106% | 108% | 95% |
| C2 c=50 | 85% | 129% | 117% | 86% |
| C3 c=1 | 57% | 65% | 63% | 59% |
| C3 c=10 | 63% | 56% | 62% | 54% |
| C3 c=50 | 56% | 51% | 66% | 48% |

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

## multihost-oracle-20260917T112107Z

| candidate | route | c | rps (med) | p95 (med) |
|---|---|---:|---:|---:|
| velqu | C0 | 1 | 12449 | 98us |
| raw-rust | C0 | 1 | 14650 | 85us |
| raw-bun | C0 | 1 | 10621 | 118us |
| elysia2 | C0 | 1 | 10568 | 118us |
| lugas | C0 | 1 | 10664 | 117us |
| velqu | C0 | 10 | 26944 | 524us |
| raw-rust | C0 | 10 | 34806 | 385us |
| raw-bun | C0 | 10 | 20596 | 768us |
| elysia2 | C0 | 10 | 21263 | 727us |
| lugas | C0 | 10 | 21997 | 680us |
| velqu | C0 | 50 | 33473 | 2115us |
| raw-rust | C0 | 50 | 42003 | 1661us |
| raw-bun | C0 | 50 | 23560 | 3202us |
| elysia2 | C0 | 50 | 23625 | 3221us |
| lugas | C0 | 50 | 23804 | 3078us |
| velqu | C1 | 1 | 12411 | 99us |
| raw-rust | C1 | 1 | 14566 | 85us |
| raw-bun | C1 | 1 | 10718 | 117us |
| elysia2 | C1 | 1 | 10381 | 121us |
| lugas | C1 | 1 | 11033 | 115us |
| velqu | C1 | 10 | 26991 | 522us |
| raw-rust | C1 | 10 | 34597 | 382us |
| raw-bun | C1 | 10 | 21592 | 700us |
| elysia2 | C1 | 10 | 20483 | 741us |
| lugas | C1 | 10 | 23800 | 640us |
| velqu | C1 | 50 | 33669 | 2048us |
| raw-rust | C1 | 50 | 42253 | 1636us |
| raw-bun | C1 | 50 | 24021 | 3084us |
| elysia2 | C1 | 50 | 22878 | 3290us |
| lugas | C1 | 50 | 24812 | 2955us |
| velqu | C2 | 1 | 12426 | 99us |
| raw-rust | C2 | 1 | 14146 | 87us |
| raw-bun | C2 | 1 | 10645 | 118us |
| elysia2 | C2 | 1 | 10607 | 117us |
| lugas | C2 | 1 | 10963 | 115us |
| velqu | C2 | 10 | 26389 | 533us |
| raw-rust | C2 | 10 | 34809 | 382us |
| raw-bun | C2 | 10 | 20792 | 727us |
| elysia2 | C2 | 10 | 20765 | 732us |
| lugas | C2 | 10 | 21787 | 689us |
| velqu | C2 | 50 | 33828 | 2040us |
| raw-rust | C2 | 50 | 41683 | 1686us |
| raw-bun | C2 | 50 | 23121 | 3242us |
| elysia2 | C2 | 50 | 22781 | 3280us |
| lugas | C2 | 50 | 23936 | 3083us |
| velqu | C3 | 1 | 6906 | 177us |
| raw-rust | C3 | 1 | 13748 | 90us |
| raw-bun | C3 | 1 | 10303 | 122us |
| elysia2 | C3 | 1 | 10086 | 123us |
| lugas | C3 | 1 | 10045 | 124us |
| velqu | C3 | 10 | 18534 | 736us |
| raw-rust | C3 | 10 | 33536 | 392us |
| raw-bun | C3 | 10 | 19908 | 760us |
| elysia2 | C3 | 10 | 19883 | 784us |
| lugas | C3 | 10 | 20289 | 753us |
| velqu | C3 | 50 | 24394 | 2783us |
| raw-rust | C3 | 50 | 40439 | 1740us |
| raw-bun | C3 | 50 | 22169 | 3376us |
| elysia2 | C3 | 50 | 22095 | 3538us |
| lugas | C3 | 50 | 22496 | 3366us |

### velqu as % of baseline (oracle-20260917T112107Z, median rps)

| route/c | raw-rust | raw-bun | elysia2 | lugas |
|---|---:|---:|---:|---:|
| C0 c=1 | 85% | 117% | 118% | 117% |
| C0 c=10 | 77% | 131% | 127% | 122% |
| C0 c=50 | 80% | 142% | 142% | 141% |
| C1 c=1 | 85% | 116% | 120% | 112% |
| C1 c=10 | 78% | 125% | 132% | 113% |
| C1 c=50 | 80% | 140% | 147% | 136% |
| C2 c=1 | 88% | 117% | 117% | 113% |
| C2 c=10 | 76% | 127% | 127% | 121% |
| C2 c=50 | 81% | 146% | 148% | 141% |
| C3 c=1 | 50% | 67% | 68% | 69% |
| C3 c=10 | 55% | 93% | 93% | 91% |
| C3 c=50 | 60% | 110% | 110% | 108% |

