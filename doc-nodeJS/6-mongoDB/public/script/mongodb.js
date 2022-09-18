"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const mongodb_1 = __importDefault(require("mongodb"));
const MongoClient = mongodb_1.default.MongoClient;
const connexionUrl = 'mongodb://127.0.0.1:27017';
const databaseName = 'task-manager';
