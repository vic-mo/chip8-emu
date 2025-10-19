#!/bin/bash
# Script to copy CHIP-8 ROMs from the chip8-roms repository

ROMS_SOURCE="../../../chip8-roms"
ROMS_DEST="public/roms"

# Check if source ROM directory exists
if [ ! -d "$ROMS_SOURCE" ]; then
    echo "Error: CHIP-8 ROMs repository not found at $ROMS_SOURCE"
    echo "Please clone the chip8-roms repository or update ROMS_SOURCE path"
    exit 1
fi

# Create destination directories
mkdir -p "$ROMS_DEST/games"
mkdir -p "$ROMS_DEST/demos"
mkdir -p "$ROMS_DEST/programs"

# Copy ROM files
echo "Copying game ROMs..."
cp "$ROMS_SOURCE/games/"*.ch8 "$ROMS_DEST/games/" 2>/dev/null

echo "Copying demo ROMs..."
cp "$ROMS_SOURCE/demos/"*.ch8 "$ROMS_DEST/demos/" 2>/dev/null

echo "Copying program ROMs..."
cp "$ROMS_SOURCE/programs/"*.ch8 "$ROMS_DEST/programs/" 2>/dev/null

# Generate catalog
echo "Generating ROM catalog..."
cat > /tmp/generate_catalog.sh << 'EOF'
#!/bin/bash

echo '{'
echo '  "games": ['

first=true
for rom in public/roms/games/*.ch8; do
    filename=$(basename "$rom")
    name="${filename%.ch8}"

    if [ "$first" = true ]; then
        first=false
    else
        echo ','
    fi

    echo -n "    {\"name\": \"$name\", \"file\": \"games/$filename\", \"category\": \"game\"}"
done

echo ''
echo '  ],'
echo '  "demos": ['

first=true
for rom in public/roms/demos/*.ch8; do
    filename=$(basename "$rom")
    name="${filename%.ch8}"

    if [ "$first" = true ]; then
        first=false
    else
        echo ','
    fi

    echo -n "    {\"name\": \"$name\", \"file\": \"demos/$filename\", \"category\": \"demo\"}"
done

echo ''
echo '  ],'
echo '  "programs": ['

first=true
for rom in public/roms/programs/*.ch8; do
    filename=$(basename "$rom")
    name="${filename%.ch8}"

    if [ "$first" = true ]; then
        first=false
    else
        echo ','
    fi

    echo -n "    {\"name\": \"$name\", \"file\": \"programs/$filename\", \"category\": \"program\"}"
done

echo ''
echo '  ]'
echo '}'
EOF

bash /tmp/generate_catalog.sh > "$ROMS_DEST/catalog.json"

# Count ROMs
GAMES_COUNT=$(ls "$ROMS_DEST/games/"*.ch8 2>/dev/null | wc -l)
DEMOS_COUNT=$(ls "$ROMS_DEST/demos/"*.ch8 2>/dev/null | wc -l)
PROGRAMS_COUNT=$(ls "$ROMS_DEST/programs/"*.ch8 2>/dev/null | wc -l)

echo ""
echo "ROM setup complete!"
echo "  Games: $GAMES_COUNT"
echo "  Demos: $DEMOS_COUNT"
echo "  Programs: $PROGRAMS_COUNT"
echo "  Total: $((GAMES_COUNT + DEMOS_COUNT + PROGRAMS_COUNT)) ROMs"
