import Foundation

@main
struct Part2IndividualTest {
    static func main() {
        let SAMPLE_INPUT = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        """

        print("Testing Part 2 implementations individually\n")
        print(String(repeating: "=", count: 60))

        // Test each Part 2 function one at a time
        let tests: [(String, (String) throws -> Int32)] = [
            ("1. Rust native", uniffiProcessPart2),
            ("2. C-style", uniffiProcessPart2C),
            ("3. Binary search", uniffiProcessPart2Bsearch),
            ("4. FreqMap", uniffiProcessPart2Freqmap),
            ("5. Real C (count)", uniffiProcessPart2GlibcCount),
            ("6. Real C (freqmap)", uniffiProcessPart2GlibcFreqmap),
            ("7. libc crate", uniffiProcessPart2Libc),
            ("8. uthash", uniffiProcessPart2Uthash),
        ]

        for (name, function) in tests {
            print("\nTesting: \(name)")
            do {
                let result = try function(SAMPLE_INPUT)
                print("   Result: \(result)")
                assert(result == 31, "Expected 31, got \(result)")
                print("   ✓ Passed")
            } catch {
                print("   ✗ Error: \(error)")
                exit(1)
            }
        }

        print("\n" + String(repeating: "=", count: 60))
        print("✓ All Part 2 tests passed!\n")
    }
}
