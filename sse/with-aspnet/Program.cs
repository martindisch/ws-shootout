using Lib.AspNetCore.ServerSentEvents;
using WithAspNet;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddHostedService<EventService>();
builder.Services.AddServerSentEvents();

var app = builder.Build();

app.MapServerSentEvents("/");

app.Run();
