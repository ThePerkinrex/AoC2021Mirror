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
			ArrayList<Long> scores = new ArrayList<Long>();
			for (int i = 0; i < nums.size(); i++) {
				long count = 0;
				String s = nums.get(i).trim();
				char[] stack = new char[s.length()];
				int size = 0;
				int j = 0;
				for (; j < s.length(); j++) {
					char c = s.charAt(j);
					if ("([{<".indexOf(c) != -1) {
						stack[size++] = c;
					} else {
						char x = stack[--size];
						if ((x == '(' && c != ')') || (x != '(' && c - x != 2)) {
							break;
						}
					}
				}
				if (j == s.length()) {
					for (int k = size-1; k >= 0; k--){
						// System.out.print("([{<".indexOf(stack[k]));
						// System.out.print(":");
						count = count * 5 + "([{<".indexOf(stack[k]) + 1;
					}
					// System.out.println();
					scores.add(count);
				}
			}
			scores.sort(null);
			// System.out.println(scores.toString());
			
			System.out.println(scores.get(scores.size()/2));
			sc.close(); // closes the scanner
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}
