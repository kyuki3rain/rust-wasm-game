FROM --platform=linux/amd64 mcr.microsoft.com/vscode/devcontainers/base:bullseye

ARG CHROME_VERSION="120.0.6099.109"
ENV CHROME_VERSION $CHROME_VERSION

RUN apt-get update \
      && apt-get upgrade -y \
      && apt-get install -y \
      ca-certificates \
      fonts-liberation \
      libasound2 \
      libatk-bridge2.0-0 \
      libatk1.0-0 \
      libatspi2.0-0 \
      libc6 \
      libcairo2 \
      libcups2 \
      libcurl3-gnutls \
      libcurl3-nss \
      libcurl4 \
      libdbus-1-3 \
      libdbus-1-3 \
      libdrm2 \
      libexpat1 \
      libgbm1 \
      libglib2.0-0 \
      libgtk-3-0 \
      libnspr4 \
      libnss3 \
      libpango-1.0-0 \
      libu2f-udev \
      libvulkan1 \
      libx11-6 \
      libxcb1 \
      libxcomposite1 \
      libxdamage1 \
      libxext6 \
      libxfixes3 \
      libxkbcommon0 \
      libxrandr2 \
      vim

# Install chrome + chromedriver
RUN mkdir -p /opt/google/chrome/ \
      && cd /opt/google/chrome/ \
      && curl -L --proto '=https' --tlsv1.2 --output /tmp/chromedriver.zip \
      "https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/120.0.6099.109/linux64/chromedriver-linux64.zip" \
      && unzip /tmp/chromedriver.zip \
      && rm /tmp/chromedriver.zip \
      && ln -s /opt/google/chrome/chromedriver-linux64/chromedriver /usr/bin/chromedriver \
      && curl -L --proto '=https' --tlsv1.2 --output /tmp/chrome.zip \
      "https://edgedl.me.gvt1.com/edgedl/chrome/chrome-for-testing/120.0.6099.109/linux64/chrome-linux64.zip" \
      && unzip /tmp/chrome.zip \
      && ln -s /opt/google/chrome/chrome-linux64/chrome /usr/bin/google-chrome-stable \
      && ln -s /usr/bin/google-chrome-stable /usr/bin/google-chrome \
      && rm /tmp/chrome.zip
