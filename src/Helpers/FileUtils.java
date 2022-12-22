package Helpers;

import java.nio.file.Files;
import java.nio.file.Paths;

public class FileUtils {
    public static String fileToString(String filePath) {
        try {
            return new String(Files.readAllBytes(Paths.get(filePath)));
        } catch (Exception e) {
            System.out.println("oops");
            return "";
        }
    }
}
