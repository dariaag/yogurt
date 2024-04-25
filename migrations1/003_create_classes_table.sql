CREATE TABLE classes (
    class_id SERIAL PRIMARY KEY,
    studio_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    instructor_name VARCHAR(255),
    start_time TIMESTAMP NOT NULL,
    duration INTEGER NOT NULL,
    capacity INTEGER NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studio_id) REFERENCES studios(studio_id)
);
