# \DnsZoneApi

All URIs are relative to *https://api.netlify.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**configure_dns_for_site**](DnsZoneApi.md#configure_dns_for_site) | **PUT** /sites/{site_id}/dns | 
[**create_dns_record**](DnsZoneApi.md#create_dns_record) | **POST** /dns_zones/{zone_id}/dns_records | 
[**create_dns_zone**](DnsZoneApi.md#create_dns_zone) | **POST** /dns_zones | 
[**delete_dns_record**](DnsZoneApi.md#delete_dns_record) | **DELETE** /dns_zones/{zone_id}/dns_records/{dns_record_id} | 
[**delete_dns_zone**](DnsZoneApi.md#delete_dns_zone) | **DELETE** /dns_zones/{zone_id} | 
[**get_dns_for_site**](DnsZoneApi.md#get_dns_for_site) | **GET** /sites/{site_id}/dns | 
[**get_dns_records**](DnsZoneApi.md#get_dns_records) | **GET** /dns_zones/{zone_id}/dns_records | 
[**get_dns_zone**](DnsZoneApi.md#get_dns_zone) | **GET** /dns_zones/{zone_id} | 
[**get_dns_zones**](DnsZoneApi.md#get_dns_zones) | **GET** /dns_zones | 
[**get_individual_dns_record**](DnsZoneApi.md#get_individual_dns_record) | **GET** /dns_zones/{zone_id}/dns_records/{dns_record_id} | 
[**transfer_dns_zone**](DnsZoneApi.md#transfer_dns_zone) | **PUT** /dns_zones/{zone_id}/transfer | 



## configure_dns_for_site

> Vec<crate::models::DnsZone> configure_dns_for_site(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::DnsZone>**](dnsZone.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dns_record

> crate::models::DnsRecord create_dns_record(zone_id, dns_record)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** |  | [required] |
**dns_record** | [**DnsRecordCreate**](DnsRecordCreate.md) |  | [required] |

### Return type

[**crate::models::DnsRecord**](dnsRecord.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dns_zone

> crate::models::DnsZone create_dns_zone(dns_zone_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dns_zone_params** | [**DnsZoneSetup**](DnsZoneSetup.md) |  | [required] |

### Return type

[**crate::models::DnsZone**](dnsZone.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dns_record

> delete_dns_record(zone_id, dns_record_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** |  | [required] |
**dns_record_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dns_zone

> delete_dns_zone(zone_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dns_for_site

> Vec<crate::models::DnsZone> get_dns_for_site(site_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::DnsZone>**](dnsZone.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dns_records

> Vec<crate::models::DnsRecord> get_dns_records(zone_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** |  | [required] |

### Return type

[**Vec<crate::models::DnsRecord>**](dnsRecord.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dns_zone

> crate::models::DnsZone get_dns_zone(zone_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** |  | [required] |

### Return type

[**crate::models::DnsZone**](dnsZone.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dns_zones

> Vec<crate::models::DnsZone> get_dns_zones(account_slug)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_slug** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::DnsZone>**](dnsZone.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_individual_dns_record

> crate::models::DnsRecord get_individual_dns_record(zone_id, dns_record_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** |  | [required] |
**dns_record_id** | **String** |  | [required] |

### Return type

[**crate::models::DnsRecord**](dnsRecord.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_dns_zone

> crate::models::DnsZone transfer_dns_zone(zone_id, account_id, transfer_account_id, transfer_user_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**zone_id** | **String** |  | [required] |
**account_id** | **String** | the account of the dns zone | [required] |
**transfer_account_id** | **String** | the account you want to transfer the dns zone to | [required] |
**transfer_user_id** | **String** | the user you want to transfer the dns zone to | [required] |

### Return type

[**crate::models::DnsZone**](dnsZone.md)

### Authorization

[netlifyAuth](../README.md#netlifyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

