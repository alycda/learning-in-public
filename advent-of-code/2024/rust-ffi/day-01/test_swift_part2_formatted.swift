import Foundation

@main
struct Part2FormattedTest {
    static func main() {
        let SAMPLE_INPUT = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        """

        print("Testing Part 2 with formatted output (like original test)\n")
        print(String(repeating: "=", count: 60))

        // Test using the EXACT same pattern as the original test
        let implementations: [(String, (String) throws -> Int32)] = [
            ("Rust (native)", uniffiProcessPart2),
            ("C-style", uniffiProcessPart2C),
            ("Binary search", uniffiProcessPart2Bsearch),
            ("FreqMap (simulated)", uniffiProcessPart2Freqmap),
            ("Real C (count)", uniffiProcessPart2GlibcCount),
            ("Real C (freqmap)", uniffiProcessPart2GlibcFreqmap),
            ("libc crate", uniffiProcessPart2Libc),
            ("uthash", uniffiProcessPart2Uthash)
        ]

        var results: [Int32] = []
        for (name, function) in implementations {
            do {
                let result = try function(SAMPLE_INPUT)
                results.append(result)
                print(String(format: "  %-25s %d", name, result))
            } catch {
                print(String(format: "  %-25s Error: %@", name, "\(error)"))
                exit(1)
            }
        }

        // Verify all results match
        let expected: Int32 = 31
        if results.allSatisfy({ $0 == expected }) {
            print("\n  ✓ All Part 2 implementations return \(expected)")
        } else {
            print("\n  ✗ Results don't match! Expected \(expected), got \(Set(results))")
            exit(1)
        }

        print("\n" + String(repeating: "=", count: 60))
        print("✓ All tests passed!\n")
    }
}
