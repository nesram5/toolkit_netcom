INSERT INTO tests (test_name, src_address, dst_address)
VALUES ('TD test', '0.0.0.0', '8.8.8.8');

INSERT INTO latency_reports (test_id, latency_ms, packet_loss)
VALUES (1, 12.345, 0);


CREATE TABLE tests (
    test_id INT AUTO_INCREMENT PRIMARY KEY,  -- Unique ID for each test
    test_name VARCHAR(255) NOT NULL,         -- Name of the test
    src_address VARCHAR(255) NOT NULL,       -- Source address (e.g., IP or hostname)
    dst_address VARCHAR(255) NOT NULL,       -- Destination address (e.g., IP or hostname)
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Timestamp of the test creation
);

CREATE TABLE latency_reports (
    report_id INT AUTO_INCREMENT PRIMARY KEY,
    test_id INT NOT NULL,
    latency_ms DECIMAL(10, 3),
    packet_loss DECIMAL(10, 3),
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (test_id) REFERENCES tests(test_id) ON DELETE CASCADE
);

CREATE INDEX idx_test_id ON latency_reports (test_id);
CREATE INDEX idx_timestamp ON latency_reports (timestamp);