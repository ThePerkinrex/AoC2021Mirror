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
			ArrayList<Integer> nums = new ArrayList<Integer>();
			// returns true if there is another line to read
			while (sc.hasNextInt()) {
				nums.add(sc.nextInt()); // returns the line that was skipped
			}
			int sum = 0;
			for (int i = 3; i < nums.size(); i++) {
				if (nums.get(i-3)+nums.get(i-2)+nums.get(i-1) < nums.get(i-2)+nums.get(i-1)+nums.get(i)) {
					sum++;
				}
			}
			System.out.println(sum);
			sc.close(); // closes the scanner
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}
