//
//  NSCTextDecoder.swift
//  WinterCG
//
//  Created by Osei Fortune on 09/01/2024.
//

import Foundation

@objcMembers
@objc(NSCTextDecoder)
public class NSCTextDecoder: NSObject {
    
    private var decoder: OpaquePointer? = nil
    public override init() {
        super.init()
       decoder = wcg_core_text_decoder_create(nil)
    }
    
    public init(encoding: String){
        super.init()
        decoder = wcg_core_text_decoder_create(encoding.cString(using: .utf8))
    }
    
    
    public var encoding: String {
        let raw = wcg_core_text_decoder_get_encoding(decoder)
        if(raw == nil){
            return ""
        }
        let encoding = String(cString: raw!)
        let raw_mut  = UnsafeMutablePointer(mutating: raw)
        wcg_core_string_destroy(raw_mut)
        return encoding
    }
    

    

    
    public func decode(_ data: [UInt8]) -> String {
        var data = data
        let decoded = wcg_core_text_decoder_decode(decoder!, &data, UInt(data.count))
        
        if(decoded == nil){
            return ""
        }
        let ret = String(cString: decoded!)
        let decoded_mut  = UnsafeMutablePointer(mutating: decoded)
        wcg_core_string_destroy(decoded_mut)
        return ret
    }
    
    public func decodeBuffer(_ buffer: Data) -> String {
        var data = buffer
        let decoded = wcg_core_text_decoder_decode(decoder!, &data, UInt(data.count))
        
        if(decoded == nil){
            return ""
        }
        let ret = String(cString: decoded!)
        let decoded_mut  = UnsafeMutablePointer(mutating: decoded)
        wcg_core_string_destroy(decoded_mut)
        return ret
    }
        
    
    deinit {
        wcg_core_text_decoder_destroy(decoder)
    }
    
}
