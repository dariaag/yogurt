CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE studios (
    studio_id SERIAL PRIMARY KEY,
    owner_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    address TEXT,
    phone VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE TABLE classes (
    class_id SERIAL PRIMARY KEY,
    studio_id INTEGER NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    instructor_name VARCHAR(255),
    start_time TIMESTAMPTZ NOT NULL,
    duration INTEGER NOT NULL,
    capacity INTEGER NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (studio_id) REFERENCES studios(studio_id)
);

CREATE TABLE bookings (
    booking_id SERIAL PRIMARY KEY,
    class_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    booking_time TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(50) NOT NULL,
    FOREIGN KEY (class_id) REFERENCES classes(class_id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Optional Payment Table
CREATE TABLE payments (
    payment_id SERIAL PRIMARY KEY,
    booking_id INTEGER NOT NULL,
    amount NUMERIC(10, 2) NOT NULL,
    payment_method VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL,
    payment_date TIMESTAMPTZ NOT NULL,
    FOREIGN KEY (booking_id) REFERENCES bookings(booking_id)
);
