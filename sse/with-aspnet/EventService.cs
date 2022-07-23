using Lib.AspNetCore.ServerSentEvents;

namespace WithAspNet;

public class EventService : BackgroundService
{
    private readonly IServerSentEventsService _serverSentEventsService;

    public EventService(IServerSentEventsService serverSentEventsService)
    {
        _serverSentEventsService = serverSentEventsService;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        while (!stoppingToken.IsCancellationRequested)
        {
            await _serverSentEventsService.SendEventAsync("myello", stoppingToken);
            await Task.Delay(TimeSpan.FromSeconds(1), stoppingToken);
        }
    }
}
