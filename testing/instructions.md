cd target/wasm32-wasip1/debug

```sh
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component list
```

```sh
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component add
```

```sh
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component list
```

```sh
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component list                                                    867ms  Mon  3 Feb 17:02:44 2025

+----------------------------------------------------+------------+---------+----------+---------------+
| URN                                                | Name       | Version | Size     | Exports count |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:9cd172f5-f2ec-4ff4-b872-9695930e7c4a | centroid   |       0 | 14703243 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:9cd172f5-f2ec-4ff4-b872-9695930e7c4a | centroid   |       1 | 14703243 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:502e9b39-7e09-4051-b1be-5e12b9840ca2 | cluster    |       0 |  3121930 |             2 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:502e9b39-7e09-4051-b1be-5e12b9840ca2 | cluster    |       1 |  3139136 |             2 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:6e6ad88a-b253-4200-b8a6-7d6a08c74c57 | embeddings |       0 | 17007599 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:6e6ad88a-b253-4200-b8a6-7d6a08c74c57 | embeddings |       1 | 17007599 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:0936ffc2-be5c-4c03-83df-fdc05541bc36 | llm        |       0 | 17632257 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:0936ffc2-be5c-4c03-83df-fdc05541bc36 | llm        |       1 | 17632257 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678 | raw        |       0 | 16540883 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678 | raw        |       1 | 16550995 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
```

## Starting the threat intel worker

```sh
/Users/afsalthaj/projects/resolve/golem/target/debug/golem worker add --worker-name security-threat-intel-v3 --component urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678 --env LLM_COMPONENT_ID=0936ffc2-be5c-4c03-83df-fdc05541bc36 --env CENTROID_COMPONENT_ID=9cd172f5-f2ec-4ff4-b872-9695930e7c4a --env EMBEDDER_COMPONENT_ID=6e6ad88a-b253-4200-b8a6-7d6a08c74c57 --env CLUSTER_COMPONENT_ID=502e9b39-7e09-4051-b1be-5e12b9840ca2
```

```sh
Added worker security-threat-intel

Worker URN:    urn:worker:e411658d-7180-40c7-aad9-bb71ffa5e678/security-threat-intel
Component URN: urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678
Worker name:   security-threat-intel

```


### Invoking functions from the beginning
#### API definition add

```sh
/Users/afsalthaj/projects/resolve/golem/target/debug/golem api-definition add api-definition/definition.yaml --def-format yaml

```

```shell
Added API definition threat-intelligence-api with version 0.0.4

ID:         threat-intelligence-api
Version:    0.0.4
Created at: 2025-02-03 11:45:15.086599 UTC
Draft:      true
Routes:
  +--------+-------------+----------------------------------------------------+----------------------------+
  | Method | Path        | Component URN                                      | Worker Name                |
  +--------+-------------+----------------------------------------------------+----------------------------+
  | Post   | /v4/add-log | urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678 | "security-threat-intel-v3" |
  +--------+-------------+----------------------------------------------------+----------------------------+
  +--------+-------------+----------------------------------------------------+-

```

#### API definition deploy

```sh
 /Users/afsalthaj/projects/resolve/golem/target/debug/golem api-deployment deploy --host localhost:9006 --definition threat-intelligence-api/0.0.4
```

```shell
API deployment on localhost:9006 with definition threat-intelligence-api/0.0.1
API deployment on localhost:9006 with definition threat-intelligence-api/0.0.2
API deployment on localhost:9006 with definition threat-intelligence-api/0.0.3
API deployment on localhost:9006 with definition threat-intelligence-api/0.0.4
```


### Testing the API

```sh
curl -X POST http://localhost:9006/v4/add-log -d '{"log" : "this is log"}'                                                                          Mon  3 Feb 17:14:03 2025

Log processed successfully
```

The pipeline needs to be tested with more number of logs, until we hit the batch size etc

