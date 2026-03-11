<<<<<<< HEAD
CREATE TABLE IF NOT EXISTS benchmark_results (
                                                 id BIGSERIAL PRIMARY KEY,
                                                 event_id TEXT NOT NULL,
                                                 format TEXT NOT NULL,
                                                 payload_size_bytes INT NOT NULL,
                                                 sent_time_ms BIGINT NOT NULL,
                                                 receive_time_ms BIGINT NOT NULL,
                                                 latency_ms BIGINT NOT NULL,
                                                 created_at TIMESTAMP DEFAULT NOW()
=======
CREATE TABLE IF NOT EXISTS benchmark_results (
                                                 id BIGSERIAL PRIMARY KEY,
                                                 event_id TEXT NOT NULL,
                                                 format TEXT NOT NULL,
                                                 payload_size_bytes INT NOT NULL,
                                                 sent_time_ms BIGINT NOT NULL,
                                                 receive_time_ms BIGINT NOT NULL,
                                                 latency_ms BIGINT NOT NULL,
                                                 created_at TIMESTAMP DEFAULT NOW()
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
    );