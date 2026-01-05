import Foundation

@main
struct ResultTest {
    static func main() {
        print("Testing Result-returning Swift bindings...")

        let SAMPLE_INPUT = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        """

        // Test calling a Result<i32, AocError> function
        print("\nTest: uniffiProcessPart1()")
        do {
            let result = try uniffiProcessPart1(input: SAMPLE_INPUT)
            print("Result: \(result)")
            assert(result == 11, "Expected 11")
            print("✓ Test passed")
        } catch {
            print("✗ Error: \(error)")
            exit(1)
        }

        print("\n✓ Result test passed!")
    }
}
