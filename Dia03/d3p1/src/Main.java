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
			ArrayList<String> lines = new ArrayList<String>();
			// returns true if there is another line to read
			while (sc.hasNextLine()) {
				lines.add(sc.nextLine().trim()); // returns the line that was skipped
			}
			char[] gamma = new char[lines.get(0).length()];
			char[] epsilon = new char[gamma.length];
			gamma[0] = '0';
			epsilon[0] = '1';
			int count = 0;
			for (int i = 1; i < gamma.length; i++) {
				gamma[i] = '0';
				epsilon[i] = '1';
				for(String line : lines) {
					// System.out.println(line.trim().charAt(i));
					if (line.trim().charAt(i) == '1') {
						count += 1;
						if (count >= lines.size() / 2) {
							gamma[i] = '1';
							epsilon[i] = '0';
							break;
						}
					}
				}
				count = 0;
			}
			// System.out.println(new String(gamma));System.out.println(new String(epsilon));
			int g = Integer.parseInt(new String(gamma), 2);
			int e = Integer.parseInt(new String(epsilon), 2);
			// System.out.println(g);System.out.println(e);
			System.out.println(g * e);
			sc.close(); // closes the scanner
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}
