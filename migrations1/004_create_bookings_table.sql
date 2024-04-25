
CREATE TABLE bookings (
    booking_id SERIAL PRIMARY KEY,
    class_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    booking_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(50) NOT NULL,
    FOREIGN KEY (class_id) REFERENCES classes(class_id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

