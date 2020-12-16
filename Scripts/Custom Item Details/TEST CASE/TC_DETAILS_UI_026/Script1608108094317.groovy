import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('Custom Item Details/UTILITIES/OPEN_BROWSER'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementVisible(findTestObject('Item Details Objects/Page_diagnostics/a_Accept Cookies'), 0)

WebUI.click(findTestObject('Item Details Objects/Page_diagnostics/a_Accept Cookies'))

WebUI.waitForElementVisible(findTestObject('Item Details Objects/Page_diagnostics/Page_diagnostics/span_Shelf_life'), 0)

WebUI.verifyElementPresent(findTestObject('Item Details Objects/Page_diagnostics/Page_diagnostics/span_Shelf_life'), 0)

WebUI.callTestCase(findTestCase('Custom Item Details/UTILITIES/CLOSE_BROWSER'), [:], FailureHandling.STOP_ON_FAILURE)
