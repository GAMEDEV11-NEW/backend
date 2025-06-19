const { MongoClient } = require('mongodb');

async function testMongoDBConnection() {
    const uri = process.env.MONGODB_URI || 'mongodb://localhost:27017';
    const dbName = process.env.MONGODB_DATABASE || 'game_admin';
    
    console.log('🔍 Testing MongoDB connection...');
    console.log(`URI: ${uri}`);
    console.log(`Database: ${dbName}`);
    
    try {
        const client = new MongoClient(uri);
        await client.connect();
        
        console.log('✅ MongoDB connection successful!');
        
        const db = client.db(dbName);
        
        // Test collections
        const collections = await db.listCollections().toArray();
        console.log('📚 Available collections:', collections.map(c => c.name));
        
        // Test basic operations
        const usersCollection = db.collection('users');
        
        // Insert a test user
        const testUser = {
            user_id: 'test_user_' + Date.now(),
            username: 'TestUser',
            status: 'Online',
            created_at: new Date(),
            updated_at: new Date(),
            login_count: 1,
            is_active: true
        };
        
        const insertResult = await usersCollection.insertOne(testUser);
        console.log('👤 Test user created:', insertResult.insertedId);
        
        // Find the test user
        const foundUser = await usersCollection.findOne({ user_id: testUser.user_id });
        console.log('🔍 Found user:', foundUser ? 'Yes' : 'No');
        
        // Clean up - delete test user
        await usersCollection.deleteOne({ user_id: testUser.user_id });
        console.log('🧹 Test user cleaned up');
        
        await client.close();
        console.log('✅ MongoDB test completed successfully!');
        
    } catch (error) {
        console.error('❌ MongoDB connection failed:', error.message);
        process.exit(1);
    }
}

// Run the test
testMongoDBConnection(); 