# Builder ---------------------------------------------------------------------
FROM mcr.microsoft.com/dotnet/sdk:6.0 AS builder
WORKDIR /app

# Stage 1: install & compile dependencies (only runs when dependencies changed)
COPY *.csproj ./
RUN dotnet restore

# Stage 2: copy code & build (only runs when code changed)
COPY . .
RUN dotnet publish -c Release -o out

# Final image -----------------------------------------------------------------
FROM mcr.microsoft.com/dotnet/aspnet:6.0
WORKDIR /app
COPY --from=builder /app/out .
ENTRYPOINT ["dotnet", "with-aspnet.dll"]
