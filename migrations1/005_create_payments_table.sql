-- Optional Payment Table
CREATE TABLE payments (
    payment_id SERIAL PRIMARY KEY,
    booking_id INTEGER NOT NULL,
    amount NUMERIC(10, 2) NOT NULL,
    payment_method VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL,
    payment_date TIMESTAMP NOT NULL,
    FOREIGN KEY (booking_id) REFERENCES bookings(booking_id)
);
