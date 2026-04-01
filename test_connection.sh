#!/bin/bash

# Test Supabase configuration
echo "🔍 Testing Supabase Configuration"
echo "=================================="

# Check if .env file exists
if [ -f .env ]; then
    echo "✅ .env file found"
    
    # Extract DATABASE_URL
    DATABASE_URL=$(grep -v '^#' .env | grep 'DATABASE_URL=' | cut -d'=' -f2- | tr -d "'\"")
    
    if [ -z "$DATABASE_URL" ]; then
        echo "❌ DATABASE_URL not found in .env"
        exit 1
    fi
    
    echo "✅ DATABASE_URL is set"
    
    # Check format
    if [[ $DATABASE_URL == postgresql://* ]]; then
        echo "✅ DATABASE_URL has correct PostgreSQL format"
    else
        echo "❌ DATABASE_URL should start with 'postgresql://'"
        exit 1
    fi
    
    # Extract components
    if [[ $DATABASE_URL =~ postgresql://([^:]+):([^@]+)@([^:]+):([^/]+)/(.+) ]]; then
        USER="${BASH_REMATCH[1]}"
        PASS="${BASH_REMATCH[2]}"
        HOST="${BASH_REMATCH[3]}"
        PORT="${BASH_REMATCH[4]}"
        DB="${BASH_REMATCH[5]}"
        
        echo ""
        echo "Connection String Components:"
        echo "  User: $USER"
        echo "  Host: $HOST"
        echo "  Port: $PORT"
        echo "  Database: $DB"
        echo "  Password: ****** (${#PASS} chars)"
        
        if [[ $HOST == *"supabase.co"* ]]; then
            echo "✅ Host appears to be Supabase"
        fi
    else
        echo "❌ Could not parse DATABASE_URL format"
        exit 1
    fi
    
    # Check HSK_PASSWORD
    HSK_PASSWORD=$(grep -v '^#' .env | grep 'HSK_PASSWORD=' | cut -d'=' -f2- | tr -d "'\"")
    if [ -z "$HSK_PASSWORD" ]; then
        echo "⚠️  HSK_PASSWORD not set (using default: hsk2025)"
    else
        echo "✅ HSK_PASSWORD is set"
    fi
    
    echo ""
    echo "📝 Next steps:"
    echo "1. Build the project: cargo build --release"
    echo "2. Run locally: cargo run"
    echo "3. Deploy to Render: https://render.com"
    echo "4. Add environment variables to Render dashboard"
    
else
    echo "❌ .env file not found"
    echo "Run: cp .env.example .env"
    exit 1
fi
