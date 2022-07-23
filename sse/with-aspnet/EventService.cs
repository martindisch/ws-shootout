using System.Globalization;
using Lib.AspNetCore.ServerSentEvents;

namespace WithAspNet;

public class EventService : BackgroundService
{
    private readonly IServerSentEventsService _serverSentEventsService;
    private readonly ILogger<EventService> _logger;

    public EventService(IServerSentEventsService serverSentEventsService, ILogger<EventService> logger)
    {
        _serverSentEventsService = serverSentEventsService;
        _logger = logger;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        while (!stoppingToken.IsCancellationRequested)
        {
            var now = DateTime.Now;

            var terminalTime = now.ToString("HH:mm:ss", CultureInfo.InvariantCulture);
            var eventTime = now.ToString("yyyy-mm-dd HH:MM:ss.fffffff K");

            await _serverSentEventsService.SendEventAsync(eventTime, stoppingToken);

            var clientCount = _serverSentEventsService.GetClients().Count;
            _logger.LogInformation(
                "[{time}] Sending date to {clientCount} clients",
                terminalTime,
                clientCount);

            await Task.Delay(TimeSpan.FromSeconds(1), stoppingToken);
        }
    }
}
