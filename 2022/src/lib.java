import java.nio.file.Files;
import java.nio.file.Paths;

public class FileUtils {
    public static String file_to_string(String path) {
        try {
            return new String(Files.readAllBytes(Paths.get(path)));
        } catch (Exception e) {
            System.out.println("oops");
            return "";
        }
    }
}
