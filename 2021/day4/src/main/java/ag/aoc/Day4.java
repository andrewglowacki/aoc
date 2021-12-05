package ag.aoc;

import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Day4 {

    public static void partOne(String file) throws FileNotFoundException, IOException {
        try (BufferedReader reader = new BufferedReader(new FileReader(file))) {
            
        }

        System.out.println("Part 1: incomplete");
    }
    
    public static void partTwo(String file) throws FileNotFoundException, IOException {
        try (BufferedReader reader = new BufferedReader(new FileReader(file))) {
            
        }

        System.out.println("Part 2: incomplete");
    }

    public static void main(String[] args) throws FileNotFoundException, IOException {
        partOne("input.txt");
        partOne("input.txt");
    }

    public static Input parseInput(BufferedReader reader) throws IOException {
        Input input = new Input();

        // parse called numbers
        Arrays.stream(reader.readLine().split(","))
            .map(Integer::parseInt)
            .forEach(input.getCalledNumbers()::add);

        reader.readLine();

        String line = null;
        while ((line = reader.readLine()) != null) {
        }
        return null;

    }

    public static class Input {
        private final Map<Integer, Set<Integer>> numberToLines = new HashMap<>();
        private final List<Set<Integer>> allLines = new ArrayList<>();
        private final List<Integer> calledNumbers = new ArrayList<>();


        public List<Integer> getCalledNumbers() {
            return calledNumbers;
        }
    }

}