import java.io.FileInputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Scanner;

public class Main {
	public static void main(String[] args) {
		try {
			// the file to be opened for reading
			FileInputStream fis = new FileInputStream("../../input.txt");
			Scanner sc = new Scanner(fis); // file to be scanned
			ArrayList<String> nums = new ArrayList<String>();
			// returns true if there is another line to read
			while (sc.hasNextLine()) {
				nums.add(sc.nextLine()); // returns the line that was skipped
			}
			int count = 0;
			for (int i = 0; i < nums.size(); i++) {
				String s = nums.get(i).trim();
				char[] stack = new char[s.length()];
				int size = 0;
				for (int j = 0; j < s.length(); j++) {
					char c = s.charAt(j);
					if ("([{<".indexOf(c) != -1) {
						stack[size++] = c;
					} else {
						char x = stack[--size];
						if ((x == '(' && c != ')') || (x != '(' && c - x != 2)) {
							if (c == ')') {
								count += 3;
							} else if (c == ']') {
								count += 57;
							} else if (c == '}') {
								count += 1197;
							} else if (c == '>') {
								count += 25137;
							}
							break;
						}
					}

				}
			}
			System.out.println(count);
			sc.close(); // closes the scanner
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}
