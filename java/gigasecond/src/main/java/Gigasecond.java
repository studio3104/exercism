import java.time.LocalDate;
import java.time.LocalDateTime;

public class Gigasecond {
    private final LocalDateTime localDateTime;

    public Gigasecond(LocalDate moment) {
        localDateTime = LocalDateTime.of(
                moment.getYear(), moment.getMonth(), moment.getDayOfMonth(),
                0, 0, 0
        ).plusSeconds(1_000_000_000);
    }

    public Gigasecond(LocalDateTime moment) {
        localDateTime = moment.plusSeconds(1_000_000_000);
    }

    public LocalDateTime getDateTime() {
        return localDateTime;
    }
}
