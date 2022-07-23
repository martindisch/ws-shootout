namespace WithAspNet;

public class EventService : BackgroundService
{
    private readonly ILogger<EventService> logger;

    public EventService(ILogger<EventService> logger)
    {
        this.logger = logger;
    }

    public override async Task StopAsync(CancellationToken cancellationToken)
    {
        // Signal cancellation to the executing method & wait until it completes or the stop token triggers
        await base.StopAsync(cancellationToken);

        // Throw if cancellation was triggered (shutdown is no longer graceful)
        cancellationToken.ThrowIfCancellationRequested();
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        while (!stoppingToken.IsCancellationRequested)
        {
            logger.LogInformation("Got executed");
            await Task.Delay(TimeSpan.FromSeconds(1), stoppingToken);
        }
    }
}
