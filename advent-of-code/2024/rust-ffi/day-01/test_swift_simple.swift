import Foundation

@main
struct SimpleTest {
    static func main() {
        print("Testing simple Swift bindings...")

        // Test 1: Simple function that returns 42
        print("\nTest 1: uniffiTestSimple()")
        let result1 = uniffiTestSimple()
        print("Result: \(result1)")
        assert(result1 == 42, "Expected 42")
        print("✓ Test 1 passed")

        // Test 2: String length
        print("\nTest 2: uniffiTestEchoLength()")
        let result2 = uniffiTestEchoLength(input: "hello")
        print("Result: \(result2)")
        assert(result2 == 5, "Expected 5")
        print("✓ Test 2 passed")

        print("\n✓ All simple tests passed!")
    }
}
