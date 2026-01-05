import Foundation

@main
struct Part1Test {
    static func main() {
        let SAMPLE_INPUT = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        """

        print("Testing Part 1 implementations\n")
        print(String(repeating: "=", count: 60))

        do {
            print("\n1. uniffiProcessPart1 (Rust native)")
            let result1 = try uniffiProcessPart1(input: SAMPLE_INPUT)
            print("   Result: \(result1)")
            assert(result1 == 11)
            print("   ✓ Passed")

            print("\n2. uniffiProcessPart1C (C qsort)")
            let result2 = try uniffiProcessPart1C(input: SAMPLE_INPUT)
            print("   Result: \(result2)")
            assert(result2 == 11)
            print("   ✓ Passed")

            print("\n3. uniffiProcessPart1Libc (libc crate)")
            let result3 = try uniffiProcessPart1Libc(input: SAMPLE_INPUT)
            print("   Result: \(result3)")
            assert(result3 == 11)
            print("   ✓ Passed")

            print("\n" + String(repeating: "=", count: 60))
            print("✓ All Part 1 tests passed!\n")

        } catch {
            print("✗ Error: \(error)")
            exit(1)
        }
    }
}
