# Ferrite System Integration

This directory contains files needed to integrate Ferrite with your operating system as a default image viewer.

## Linux Installation

1. Copy the desktop entry:
   ```bash
   sudo cp linux/applications/ferrite.desktop /usr/share/applications/
   ```

2. Update MIME database:
   ```bash
   sudo update-desktop-database
   ```

3. Set as default for image files (Optional):
   ```bash
   xdg-mime default ferrite.desktop image/jpeg
   xdg-mime default ferrite.desktop image/png
   xdg-mime default ferrite.desktop image/gif
   xdg-mime default ferrite.desktop image/bmp
   xdg-mime default ferrite.desktop image/tiff
   xdg-mime default ferrite.desktop image/webp
   ```

## macOS Installation

1. Copy Ferrite.app to /Applications:
   ```bash
   cp -r macos/Ferrite.app /Applications/
   ```

2. To set as default viewer:
   - Right-click an image file
   - Select "Get Info"
   - Under "Open with:", select Ferrite
   - Click "Change All..." to apply to all files of that type

## Windows Installation

1. Install Ferrite to Program Files:
   ```batch
   xcopy /E /I ferrite "%ProgramFiles%\Ferrite"
   ```

2. Import registry entries:
   - Double-click `windows/registry.reg`
   - Confirm the registry modifications when prompted

3. To set as default viewer:
   - Right-click an image file
   - Select "Open with" → "Choose another app"
   - Select Ferrite from the list
   - Check "Always use this app"
   - Click "OK"

## Uninstallation

### Linux
```bash
sudo rm /usr/share/applications/ferrite.desktop
sudo rm -r /usr/share/icons/hicolor/*/apps/ferrite.png
sudo update-icon-cache /usr/share/icons/hicolor
sudo update-desktop-database
```

### macOS
```bash
rm -rf /Applications/Ferrite.app
```

### Windows
1. Run Windows Settings
2. Go to Apps → Apps & features
3. Find Ferrite and click Uninstall
4. Additionally, you can remove registry entries by running:
   ```batch
   reg delete "HKEY_CLASSES_ROOT\Applications\ferrite.exe" /f
   ```

## Notes

- For all platforms, make sure the Ferrite executable is in your system's PATH or specified with absolute path in the integration files.(If installed with cargo, it will be in cargo/bin)
- The integration files assume Ferrite is installed in standard locations. Adjust paths if you installed it elsewhere.
- Administrative/root privileges may be required for some operations.
- Back up your system before making registry modifications on Windows.