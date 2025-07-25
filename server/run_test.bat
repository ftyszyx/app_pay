
echo  Running all tests...
cargo test --release -- --nocapture

if %errorlevel% equ 0 (
    echo ✅ All tests passed!
) else (
    echo ❌ Some tests failed!
)

