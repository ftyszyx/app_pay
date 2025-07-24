
echo 🔍 Running all tests...
cargo test --release -- --nocapture

if %errorlevel% equ 0 (
    echo ✅ All tests passed!
) else (
    echo ❌ Some tests failed!
)

echo.
echo 📊 Running specific test categories:
echo.

echo 🔐 Authentication tests...
cargo test auth_tests --release -- --nocapture

echo 👥 User management tests...
cargo test user_tests --release -- --nocapture

echo 📱 App management tests...
cargo test app_tests --release -- --nocapture

echo 🎭 Role management tests...
cargo test role_tests --release -- --nocapture

echo 🛍️ Product management tests...
cargo test product_tests --release -- --nocapture

echo 💳 Payment method tests...
cargo test pay_method_tests --release -- --nocapture

echo 🌐 General tests...
cargo test general_tests --release -- --nocapture

echo ⚡ Performance tests...
cargo test performance_tests --release -- --nocapture

pause