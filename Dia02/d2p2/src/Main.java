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
			int d = 0;
			int f = 0;
			int aim = 0;
			// returns true if there is another line to read
			while (sc.hasNext()) {
				String cmd = sc.next();
				int arg = sc.nextInt();
				if (cmd.equals("up")) {
					aim -= arg;
				}else if (cmd.equals("down")) {
					aim += arg;
				}else {
					f += arg;
					d += arg * aim;
				}

			}
			System.out.println(d*f);
			sc.close(); // closes the scanner
		} catch (IOException e) {
			e.printStackTrace();
		}
	}
}