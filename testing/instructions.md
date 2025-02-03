cd target/wasm32-wasip1/debug

```angular2html
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component list
```

```angular2html
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component add
```

```angular2html
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component list
```

```angular2html
/Users/afsalthaj/projects/resolve/golem/target/debug/golem component list                                             Mon  3 Feb 15:35:27 2025
+----------------------------------------------------+------------+---------+----------+---------------+
| URN                                                | Name       | Version | Size     | Exports count |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:9cd172f5-f2ec-4ff4-b872-9695930e7c4a | centroid   |       0 | 14703243 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:502e9b39-7e09-4051-b1be-5e12b9840ca2 | cluster    |       0 |  3121930 |             2 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:6e6ad88a-b253-4200-b8a6-7d6a08c74c57 | embeddings |       0 | 17007599 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:0936ffc2-be5c-4c03-83df-fdc05541bc36 | llm        |       0 | 17632257 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
| urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678 | raw        |       0 | 16540883 |             1 |
+----------------------------------------------------+------------+---------+----------+---------------+
```

## Starting the threat intel worker

```angular2html
/Users/afsalthaj/projects/resolve/golem/target/debug/golem worker add --worker-name security-threat-intel --component urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678 --env LLM_COMPONENT_ID=0936ffc2-be5c-4c03-83df-fdc05541bc36 --env CENTROID_COMPONENT_ID=9cd172f5-f2ec-4ff4-b872-9695930e7c4a --env EMBEDDER_COMPONENT_ID=6e6ad88a-b253-4200-b8a6-7d6a08c74c57 --env CLUSTER_COMPONENT_ID=502e9b39-7e09-4051-b1be-5e12b9840ca2
```

```angular2html
Added worker security-threat-intel

Worker URN:    urn:worker:e411658d-7180-40c7-aad9-bb71ffa5e678/security-threat-intel
Component URN: urn:component:e411658d-7180-40c7-aad9-bb71ffa5e678
Worker name:   security-threat-intel

```


