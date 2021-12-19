import java.io.FileInputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
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
			int[] counts = new int[gamma.length];
			gamma[0] = '0';
			epsilon[0] = '1';
			int count = 0;
			for (String line : lines) {
				if (line.trim().charAt(0) == '1') {
					count += 1;
				}
			}
			counts[0] = count;
			count =0;
			for (int i = 1; i < gamma.length; i++) {
				gamma[i] = '0';
				epsilon[i] = '1';
				for (String line : lines) {
					// System.out.println(line.trim().charAt(i));
					if (line.trim().charAt(i) == '1') {
						count += 1;
						if (count >= lines.size() / 2) {
							gamma[i] = '1';
							epsilon[i] = '0';
						}
					}
				}
				
				counts[i] = count;
				count = 0;
			}
			// System.out.println(Arrays.toString(counts));
			ArrayList<String> co2 = (ArrayList<String>) lines.clone();
			int[] co2_counts = Arrays.copyOf(counts, counts.length);
			ArrayList<String> o2 = (ArrayList<String>) lines.clone();
			int[] o2_counts = Arrays.copyOf(counts, counts.length);

			for (int i = 0; i <= gamma.length; i++) {
				int currlen = o2.size();
				int[] currcounts = Arrays.copyOf(o2_counts, o2_counts.length);
				for (int j = o2.size() - 1; j >= 0 && o2.size() > 1; j--) {
					// System.out.println(j + ": " + o2.size());
					if ((((float) currcounts[i]) >= ((float) currlen) / 2f && o2.get(j).charAt(i) == '0')
							|| (((float) currcounts[i]) < ((float) currlen) / 2f && o2.get(j).charAt(i) == '1')) {
						for (int x = 0; x < o2.get(j).length(); x++) {
							if (o2.get(j).charAt(x) == '1')
								o2_counts[x] -= 1;
						}
						o2.remove(j);
					}
				}
				currlen = co2.size();
				currcounts = Arrays.copyOf(co2_counts, co2_counts.length);
				for (int j = co2.size() - 1; j >= 0 && co2.size() > 1; j--) {
					if ((((float) currcounts[i]) >= ((float) currlen) / 2f && co2.get(j).charAt(i) == '1')
							|| (((float) currcounts[i]) < ((float) currlen) / 2f && co2.get(j).charAt(i) == '0')) {
						for (int x = 0; x < co2.get(j).length(); x++) {
							if (co2.get(j).charAt(x) == '1')
								co2_counts[x] -= 1;
						}
						co2.remove(j);
					}
				}
			}

			// System.out.println(new String(gamma));System.out.println(new
			// String(epsilon));
			System.out.println(" o2: " + o2.get(0));
			System.out.println("co2: " + co2.get(0));
			int g = Integer.parseInt(o2.get(0), 2);
			int e = Integer.parseInt(co2.get(0), 2);
			System.out.println(" o2: " + g);
			System.out.println("co2: " + e);
			// System.out.println(g);System.out.println(e);
			System.out.println(">>> " + g * e);
			sc.close(); // closes the scanner
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}
