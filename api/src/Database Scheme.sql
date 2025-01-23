INSERT INTO tests (test_name, src_address, dst_address)
VALUES ('TD test', '0.0.0.0', '8.8.8.8');

INSERT INTO latency_reports (test_id, latency_ms)
VALUES (1, 12.34);


CREATE TABLE latency_reports (
    report_id INT AUTO_INCREMENT PRIMARY KEY,
    test_id INT NOT NULL,
    latency_ms DECIMAL(10, 2) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (test_id) REFERENCES tests(test_id) ON DELETE CASCADE
);

CREATE TABLE latency_reports (
    report_id INT AUTO_INCREMENT PRIMARY KEY,
    test_id INT NOT NULL,
    latency_ms DECIMAL(10, 2) NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (test_id) REFERENCES tests(test_id)
);