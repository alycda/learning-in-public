import Foundation

@main
struct ArrayInitTest {
    static func main() {
        print("Testing closure array initialization...")

        // Try to initialize the array like in the original test
        print("Creating closure array...")
        let implementations: [(String, (String) throws -> Int32)] = [
            ("Rust (native)", uniffiProcessPart2),
            ("C-style", uniffiProcessPart2C),
        ]

        print("Array created successfully!")
        print("Array has \(implementations.count) elements")
        print("✓ Test passed")
    }
}
